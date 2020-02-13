use serde::Serialize;

#[derive(Serialize)]
pub struct Page<T> {
  pub logo: Option<String>,
  pub title: String,
  pub home_link: String,
  pub left_menu: Vec<SubMenu>,
  #[serde(flatten)]
  pub subpage: T,
}

#[derive(Serialize)]
pub struct SubMenu {
  pub id: Option<String>,
  pub title: String,
  pub options: Vec<SubMenuItem>,
}

#[derive(Serialize)]
pub struct SubMenuItem {
  pub uri: String,
  pub title: String,
}
