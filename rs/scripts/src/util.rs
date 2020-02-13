use hydra::diesel::{self, prelude::*, PgConnection};
use hydra::{models::*, schema::*};
use std::error::Error;

pub async fn handle_declarative_build(
  project: &Project,
  build: &Build,
  connection: &PgConnection,
) -> Result<(), Box<dyn Error>> {
  let e: Result<(), Box<dyn Error>> = try {};
  if let Err(e) = e {
    diesel::update(
      jobsets::table.filter(
        jobsets::project
          .eq(&project.name)
          .and(jobsets::name.eq(".jobsets")),
      ),
    )
    .set((
      jobsets::errormsg.eq(Some(e.to_string())),
      jobsets::errortime.eq(Some(unixtime() as i32)),
      jobsets::fetcherrormsg.eq(<Option<String>>::None),
    ))
    .execute(connection)?;
  }
  Ok(())
}

pub fn unixtime() -> u64 {
  use std::time::SystemTime;
  SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs()
}
