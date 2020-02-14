use crate::plugin::*;
use reqwest::Client;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
struct Config {
  url: String,
  db: String,
}

pub struct InfluxDB {
  config: Config,
  client: Client,
}

#[async_trait]
impl PluginInit for InfluxDB {
  async fn init(hydra: &Hydra) -> Result<Option<Arc<dyn Plugin>>> {
    if let Some(config) = hydra.sub_config("influxdb") {
      Ok(Some(Arc::new(Self {
        config,
        client: Client::new(),
      })))
    } else {
      Ok(None)
    }
  }
}

#[async_trait]
impl Plugin for InfluxDB {
  async fn build_finished(&self, build: &Build, dependents: &[Build]) -> Result<()> {
    let mut payload = String::new();

    for b in Some(build).into_iter().chain(dependents) {
      make_line(
        &mut payload,
        "hydra_build_status",
        &mut [
          ("status", &status_class(b.buildstatus)),
          (
            "result",
            &b.buildstatus
              .and_then(|s| STATUS_DETAILED.get(s as usize))
              .map_or("aborted", |x| *x),
          ),
          ("project", &b.project),
          ("jobset", &b.jobset),
          ("repo", &b.jobset),
          ("job", &b.job),
          ("system", &b.system),
          ("cached", &(b.iscachedbuild == Some(1))),
        ],
        &mut [
          ("build_status", &format!("{}i", b.buildstatus.unwrap_or(0))),
          ("build_id", &format!("\"{}\"", b.id)),
          ("main_build_id", &format!("\"{}\"", build.id)),
          (
            "duration",
            &format!("{}i", b.stoptime.unwrap_or(0) - b.starttime.unwrap_or(0)),
          ),
          (
            "queued",
            &format!(
              "{}i",
              std::cmp::max(0, b.starttime.map_or(0, |x| x - b.timestamp))
            ),
          ),
          ("closure_size", &format!("{}i", b.closuresize.unwrap_or(0))),
          ("size", &format!("{}i", b.size.unwrap_or(0))),
        ],
        b.stoptime.expect("stoptime missing"),
      );
    }

    debug!("sending payload: {:?}", payload);

    let req = self
      .client
      .post(&format!(
        "{}/write?db={}&precision=s",
        &self.config.url, &self.config.db
      ))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(payload);

    debug!("{:?}", req.send().await?);

    Ok(())
  }
}

fn make_line(
  out: &mut String,
  measurement: &str,
  tags: &mut [(&str, &dyn Display)],
  fields: &mut [(&str, &dyn Display)],
  timestamp: i32,
) {
  tags.sort_unstable_by_key(|x| x.0);
  fields.sort_unstable_by_key(|x| x.0);

  out.push_str(measurement);
  for (k, v) in tags {
    out.push_str(&format!(",{}={}", k, v));
  }
  out.push(' ');
  for (i, (k, v)) in fields.iter().enumerate() {
    out.push_str(&format!("{}{}={}", if i > 0 { "," } else { "" }, k, v));
  }

  out.push_str(&format!(" {}\n", timestamp));
}

static STATUS_DETAILED: [&str; 13] = [
  "success",
  "failure",
  "dependency-failed",
  "<unused>",
  "cancelled",
  "<unused>",
  "failed-with-output",
  "timed-out",
  "<unused>",
  "unsupported-system",
  "log-limit-exceeded",
  "output-limit-exceeded",
  "non-deterministic-build",
];

fn status_class(i: Option<i32>) -> &'static str {
  i.map(|x| match x {
    0 => "success",
    3 | 4 | 8 | 10 | 11 => "canceled",
    _ => "failed",
  })
  .unwrap_or("failed")
}
