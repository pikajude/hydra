use hydra::diesel::{self, prelude::*, PgConnection};
use hydra::{models::*, schema::*, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct NewJobset {
  pub description: Option<String>,
  pub nixexprinput: String,
  pub nixexprpath: String,
  pub enabled: Option<bool>,
  pub enableemail: Option<bool>,
  pub emailoverride: String,
  pub keepnr: Option<i32>,
  pub checkinterval: Option<i32>,
  pub schedulingshares: Option<i32>,
  pub inputs: HashMap<String, NewJobsetInput>,
}

#[derive(Deserialize)]
pub struct NewJobsetInput {
  #[serde(rename = "type")]
  pub type_: String,
  pub emailresponsible: bool,
  pub value: String,
}

pub async fn handle_declarative_build(
  project: &Project,
  build: &Build,
  connection: &PgConnection,
) -> Result<()> {
  let project_ = project;

  let e: Result<()> = try {
    if build.buildstatus != Some(0) {
      Err(format!("Declarative jobset build {} failed", build.id))?
    }

    let path = BuildOutput::belonging_to(build)
      .get_result::<BuildOutput>(connection)?
      .path;
    let decl_text = std::fs::read_to_string(&path)?;
    let decl_spec = serde_json::from_str::<HashMap<String, NewJobset>>(&decl_text)?;

    let kept_items = decl_spec
      .keys()
      .cloned()
      .chain(Some(".jobsets".to_string()));

    {
      use self::jobsets::dsl::*;
      diesel::update(
        jobsets.filter(
          project
            .eq(&project_.name)
            .and(name.ne(diesel::dsl::all(kept_items.collect::<Vec<_>>()))),
        ),
      )
      .set((enabled.eq(0), hidden.eq(1)))
      .execute(connection)?;
    }

    for (jobset_name, spec) in decl_spec {
      update_declarative_jobset(project_, jobset_name, spec, connection).await?;
    }
  };

  if let Err(e) = e {
    use self::jobsets::dsl::*;
    diesel::update(jobsets.filter(project.eq(&project_.name).and(name.eq(".jobsets"))))
      .set((
        errormsg.eq(Some(e.to_string())),
        errortime.eq(Some(unixtime() as i32)),
        fetcherrormsg.eq(<Option<String>>::None),
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

pub async fn update_declarative_jobset(
  project_: &Project,
  jobset_name: String,
  spec: NewJobset,
  connection: &PgConnection,
) -> Result<()> {
  use self::jobsets::dsl::*;
  use diesel::pg::upsert::excluded;

  connection
    .transaction::<_, diesel::result::Error, _>(|| {
      let changeset = (
        project.eq(&project_.name),
        name.eq(&jobset_name),
        description.eq(spec.description),
        nixexprinput.eq(spec.nixexprinput),
        nixexprpath.eq(spec.nixexprpath),
        spec.enabled.map(|x| enabled.eq(x as i32)),
        spec.enableemail.map(|x| enableemail.eq(x as i32)),
        emailoverride.eq(spec.emailoverride),
        spec.keepnr.map(|x| keepnr.eq(x)),
        spec.checkinterval.map(|x| checkinterval.eq(x)),
        spec.schedulingshares.map(|x| schedulingshares.eq(x)),
      );

      diesel::insert_into(jobsets)
        .values(&changeset)
        .on_conflict((project, name))
        .do_update()
        .set((
          description.eq(excluded(description)),
          nixexprinput.eq(excluded(nixexprinput)),
          nixexprpath.eq(excluded(nixexprpath)),
          errormsg.eq(excluded(errormsg)),
          errortime.eq(excluded(errortime)),
          lastcheckedtime.eq(excluded(lastcheckedtime)),
          triggertime.eq(excluded(triggertime)),
          enabled.eq(excluded(enabled)),
          enableemail.eq(excluded(enableemail)),
          hidden.eq(excluded(hidden)),
          emailoverride.eq(excluded(emailoverride)),
          keepnr.eq(excluded(keepnr)),
          checkinterval.eq(excluded(checkinterval)),
          schedulingshares.eq(excluded(schedulingshares)),
          fetcherrormsg.eq(excluded(fetcherrormsg)),
          forceeval.eq(excluded(forceeval)),
          starttime.eq(excluded(starttime)),
        ))
        .execute(connection)?;

      diesel::delete(
        jobsetinputs::table.filter(
          jobsetinputs::jobset
            .eq(&jobset_name)
            .and(jobsetinputs::project.eq(&project_.name)),
        ),
      )
      .execute(connection)?;

      for (input_name, input) in spec.inputs {
        JobsetInput {
          jobset: jobset_name.clone(),
          project: project_.name.clone(),
          name: input_name.clone(),
          type_: input.type_,
          emailresponsible: input.emailresponsible as i32,
        }
        .insert_into(jobsetinputs::table)
        .execute(connection)?;

        JobsetInputAlt {
          project: project_.name.clone(),
          jobset: jobset_name.clone(),
          input: input_name,
          altnr: 0,
          value: Some(input.value),
          revision: None,
        }
        .insert_into(jobsetinputalts::table)
        .execute(connection)?;
      }

      Ok(())
    })
    .map_err(|e| e.into())
}
