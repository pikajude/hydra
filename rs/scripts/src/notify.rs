#![feature(try_blocks, type_name_of_val)]

#[macro_use]
extern crate log;

use clap::{App, Arg};
use futures::channel::mpsc;
use futures::stream::{poll_fn, Stream, StreamExt, TryStreamExt};
use hydra::diesel::prelude::*;
use hydra::{models::*, schema::*};
use hydra::{Hydra, Result};
use std::path::Path;
use tokio_postgres::{AsyncMessage, NoTls};

mod util;

async fn notify_stream() -> Result<impl Stream<Item = AsyncMessage>> {
  let (client, mut connection) =
    tokio_postgres::connect("host=/run/postgresql user=hydra", NoTls).await?;

  let (tx, rx) = mpsc::unbounded();
  tokio::spawn(
    poll_fn(move |c| connection.poll_message(c))
      .map_err(|e| panic!(e))
      .forward(tx),
  );

  client
    .batch_execute(
      "LISTEN build_started;
      LISTEN build_finished;
      LISTEN step_finished;
      LISTEN eval_job_failed;
      LISTEN eval_failed;
      LISTEN eval_finished",
    )
    .await?;

  let _client = std::mem::ManuallyDrop::new(client);

  Ok(rx)
}

struct Notify {
  hydra: Hydra,
  only: Option<String>,
  once: bool,
}

impl Notify {
  async fn run(self) -> Result<()> {
    let mut notify = notify_stream().await?;

    if self.only.as_deref().map_or(true, |x| x == "build_finished") {
      for build in builds::table
        .filter(builds::notificationpendingsince.is_not_null())
        .get_results::<Build>(&self.hydra.conn()?)?
      {
        info!("sending notifications for build {}...", build.id);
        self.build_finished(&build, vec![]).await?;
        if self.once {
          std::process::exit(0)
        }
      }
    }

    while let Some(msg) = notify.next().await {
      if let AsyncMessage::Notification(notif) = msg {
        debug!(
          "got '{}' from {}: {:?}",
          notif.channel(),
          notif.process_id(),
          notif.payload()
        );

        let mut payload = notif.payload().split('\t');

        macro_rules! arg {
          () => {
            arg!(_).parse()?
          };
          ( _ ) => {
            payload.next().expect("no argument")
          };
          ($t:ty) => {
            arg!(_).parse::<$t>()?
          };
        }

        if self.only.as_deref().map_or(false, |x| notif.channel() != x) {
          debug!("skipping as --only was passed");
          continue;
        }

        let merr: Result<()> = try {
          match notif.channel() {
            "build_started" => self.build_started(arg!()).await?,
            "build_finished" => {
              let build = builds::table
                .find(arg!(i32))
                .get_result::<Build>(&self.hydra.conn()?)?;
              self
                .build_finished(
                  &build,
                  payload
                    .filter_map(|id_str| match id_str.parse::<i32>() {
                      Ok(x) => {
                        if x == build.id {
                          None
                        } else {
                          Some(Ok(x))
                        }
                      }
                      Err(e) => Some(Err(e)),
                    })
                    .collect::<std::result::Result<Vec<i32>, _>>()?,
                )
                .await?
            }
            "step_finished" => self.step_finished(arg!(), arg!(), arg!(_)).await?,
            _ => {}
          }

          if self.once {
            std::process::exit(0);
          }
        };

        if let Err(e) = merr {
          warn!("{}", e);
        }
      }
    }

    Ok(())
  }

  async fn build_started(&self, id: i32) -> Result<()> {
    let build = builds::table
      .find(id)
      .get_result::<Build>(&self.hydra.conn()?)?;

    for plugin in self.hydra.plugins() {
      debug!("{}::build_started", plugin.name());
      if let Err(e) = plugin.build_started(&build).await {
        info!("{}::build_started: {}", plugin.name(), e);
      }
    }

    Ok(())
  }

  async fn build_finished(&self, build: &Build, dependents: Vec<i32>) -> Result<()> {
    use hydra::diesel::pg::expression::dsl::any;

    let conn = self.hydra.conn()?;

    let project = projects::table
      .find(&build.project)
      .get_result::<Project>(&conn)?;

    if project.declfile.is_some() && build.jobset == ".jobsets" && build.iscurrent == Some(1) {
      util::handle_declarative_build(&project, build, &conn).await?;
    }

    let dependents = builds::table
      .filter(builds::id.eq(any(dependents)))
      .get_results::<Build>(&conn)?;

    for plugin in self.hydra.plugins() {
      debug!("{}::build_finished", plugin.name());
      if let Err(e) = plugin.build_finished(build, &dependents).await {
        info!("{}::build_finished: {}", plugin.name(), e);
      }
    }

    hydra::diesel::update(build)
      .set(builds::notificationpendingsince.eq(<Option<i32>>::None))
      .execute(&conn)?;

    Ok(())
  }

  async fn step_finished(&self, id: i32, step_number: i32, log_path: &str) -> Result<()> {
    let conn = self.hydra.conn()?;

    let step = buildsteps::table
      .find((id, step_number))
      .get_result::<BuildStep>(&conn)?;

    let log_path = if log_path == "-" { "" } else { log_path };

    for plugin in self.hydra.plugins() {
      debug!("{}::step_finished", plugin.name());
      if let Err(e) = plugin.step_finished(&step, Path::new(log_path)).await {
        info!("{}::step_finished: {}", plugin.name(), e);
      }
    }

    Ok(())
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let matches = App::new("hydra-notify")
    .arg(
      Arg::with_name("only")
        .long("only")
        .value_name("EVENT")
        .takes_value(true),
    )
    .arg(Arg::with_name("once").long("once").takes_value(false))
    .get_matches();

  let notif = Notify {
    hydra: Hydra::get().await?,
    only: matches.value_of("only").map(|x| x.into()),
    once: matches.is_present("once"),
  };

  notif.run().await
}
