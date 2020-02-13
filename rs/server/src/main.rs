#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use self::diesel::prelude::*;
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde_json::json;

use db::models;
use db::schema::*;

mod page;

#[database("hydra")]
struct Db(diesel::PgConnection);

#[get("/")]
fn index(db: Db) -> Result<Template, Box<dyn std::error::Error>> {
  let ps = projects::table
    .order_by(projects::name)
    .get_results::<models::Project>(&*db)?;
  Ok(Template::render(
    "index",
    page::Page {
      logo: None,
      title: "Overview".into(),
      home_link: uri!(index).to_string(),
      left_menu: vec![page::SubMenu {
        id: None,
        title: "Status".into(),
        options: vec![page::SubMenuItem {
          uri: "/queue_summary".into(),
          title: "Queue (0/0)".into(),
        }],
      }],
      subpage: json!({ "projects": ps }),
    },
  ))
}

fn main() {
  let rock = rocket::ignite()
    .attach(Db::fairing())
    .attach(Template::fairing());

  let files = StaticFiles::from(rock.config().get_str("static_dir").unwrap());

  rock
    .mount("/static", files)
    .mount("/", routes![index])
    .launch();
}
