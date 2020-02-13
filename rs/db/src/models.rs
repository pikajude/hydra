#![allow(clippy::single_component_path_imports)]

use super::schema::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(aggregate, constituent)]
#[table_name = "aggregateconstituents"]
pub struct AggregateConstituent {
  pub aggregate: i32,
  pub constituent: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[table_name = "buildinputs"]
#[changeset_options(treat_none_as_null = "true")]
pub struct BuildInput {
  pub id: i32,
  pub build: Option<i32>,
  pub name: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub uri: Option<String>,
  pub revision: Option<String>,
  pub value: Option<String>,
  pub emailresponsible: i32,
  pub dependency: Option<i32>,
  pub path: Option<String>,
  pub sha256hash: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(build, name)]
#[table_name = "buildmetrics"]
#[changeset_options(treat_none_as_null = "true")]
pub struct BuildMetric {
  pub build: i32,
  pub name: String,
  pub unit: Option<String>,
  pub value: f64,
  pub project: String,
  pub jobset: String,
  pub job: String,
  pub timestamp: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(build, name)]
#[table_name = "buildoutputs"]
pub struct BuildOutput {
  pub build: i32,
  pub name: String,
  pub path: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(build, productnr)]
#[table_name = "buildproducts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct BuildProduct {
  pub build: i32,
  pub productnr: i32,
  #[serde(rename = "type")]
  pub type_: String,
  pub subtype: String,
  pub filesize: Option<i64>,
  pub sha1hash: Option<String>,
  pub sha256hash: Option<String>,
  pub path: Option<String>,
  pub name: String,
  pub defaultpath: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[table_name = "builds"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Build {
  pub id: i32,
  pub finished: i32,
  pub timestamp: i32,
  pub project: String,
  pub jobset: String,
  pub job: String,
  pub nixname: Option<String>,
  pub description: Option<String>,
  pub drvpath: String,
  pub system: String,
  pub license: Option<String>,
  pub homepage: Option<String>,
  pub maintainers: Option<String>,
  pub maxsilent: Option<i32>,
  pub timeout: Option<i32>,
  pub ischannel: i32,
  pub iscurrent: Option<i32>,
  pub nixexprinput: Option<String>,
  pub nixexprpath: Option<String>,
  pub priority: i32,
  pub globalpriority: i32,
  pub starttime: Option<i32>,
  pub stoptime: Option<i32>,
  pub iscachedbuild: Option<i32>,
  pub buildstatus: Option<i32>,
  pub size: Option<i64>,
  pub closuresize: Option<i64>,
  pub releasename: Option<String>,
  pub keep: i32,
  pub notificationpendingsince: Option<i32>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(build, stepnr, name)]
#[table_name = "buildstepoutputs"]
pub struct BuildStepOutput {
  pub build: i32,
  pub stepnr: i32,
  pub name: String,
  pub path: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(build, stepnr)]
#[table_name = "buildsteps"]
#[changeset_options(treat_none_as_null = "true")]
pub struct BuildStep {
  pub build: i32,
  pub stepnr: i32,
  #[serde(rename = "type")]
  pub type_: i32,
  pub drvpath: Option<String>,
  pub busy: i32,
  pub status: Option<i32>,
  pub errormsg: Option<String>,
  pub starttime: Option<i32>,
  pub stoptime: Option<i32>,
  pub machine: String,
  pub system: Option<String>,
  pub propagatedfrom: Option<i32>,
  pub overhead: Option<i32>,
  pub timesbuilt: Option<i32>,
  pub isnondeterministic: Option<bool>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, revision)]
#[table_name = "cachedbazaarinputs"]
pub struct CachedBazaarInput {
  pub uri: String,
  pub revision: i32,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, module, sha256hash)]
#[table_name = "cachedcvsinputs"]
pub struct CachedCvsInput {
  pub uri: String,
  pub module: String,
  pub timestamp: i32,
  pub lastseen: i32,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, revision)]
#[table_name = "cacheddarcsinputs"]
pub struct CachedDarcsInput {
  pub uri: String,
  pub revision: String,
  pub sha256hash: String,
  pub storepath: String,
  pub revcount: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, branch, revision)]
#[table_name = "cachedgitinputs"]
pub struct CachedGitInput {
  pub uri: String,
  pub branch: String,
  pub revision: String,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, branch, revision)]
#[table_name = "cachedhginputs"]
pub struct CachedHgInput {
  pub uri: String,
  pub branch: String,
  pub revision: String,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(srcpath, sha256hash)]
#[table_name = "cachedpathinputs"]
pub struct CachedPathInput {
  pub srcpath: String,
  pub timestamp: i32,
  pub lastseen: i32,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(uri, revision)]
#[table_name = "cachedsubversioninputs"]
pub struct CachedSubversionInput {
  pub uri: String,
  pub revision: i32,
  pub sha256hash: String,
  pub storepath: String,
}

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(path)]
#[table_name = "failedpaths"]
pub struct FailedPath {
  pub path: String,
}

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(project, jobset, name)]
#[table_name = "jobs"]
pub struct Job {
  pub project: String,
  pub jobset: String,
  pub name: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(eval, name, altnr)]
#[table_name = "jobsetevalinputs"]
#[changeset_options(treat_none_as_null = "true")]
pub struct JobsetEvalInput {
  pub eval: i32,
  pub name: String,
  pub altnr: i32,
  #[serde(rename = "type")]
  pub type_: String,
  pub uri: Option<String>,
  pub revision: Option<String>,
  pub value: Option<String>,
  pub dependency: Option<i32>,
  pub path: Option<String>,
  pub sha256hash: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(eval, build)]
#[table_name = "jobsetevalmembers"]
pub struct JobsetEvalMember {
  pub eval: i32,
  pub build: i32,
  pub isnew: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[table_name = "jobsetevals"]
#[changeset_options(treat_none_as_null = "true")]
pub struct JobsetEval {
  pub id: i32,
  pub project: String,
  pub jobset: String,
  pub timestamp: i32,
  pub checkouttime: i32,
  pub evaltime: i32,
  pub hasnewbuilds: i32,
  pub hash: String,
  pub nrbuilds: Option<i32>,
  pub nrsucceeded: Option<i32>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, jobset, input, altnr)]
#[table_name = "jobsetinputalts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct JobsetInputAlt {
  pub project: String,
  pub jobset: String,
  pub input: String,
  pub altnr: i32,
  pub value: Option<String>,
  pub revision: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, jobset, name)]
#[table_name = "jobsetinputs"]
pub struct JobsetInput {
  pub project: String,
  pub jobset: String,
  pub name: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub emailresponsible: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, from_)]
#[table_name = "jobsetrenames"]
pub struct JobsetRename {
  pub project: String,
  pub from_: String,
  pub to_: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, name)]
#[table_name = "jobsets"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Jobset {
  pub name: String,
  pub project: String,
  pub description: Option<String>,
  pub nixexprinput: String,
  pub nixexprpath: String,
  pub errormsg: Option<String>,
  pub errortime: Option<i32>,
  pub lastcheckedtime: Option<i32>,
  pub triggertime: Option<i32>,
  pub enabled: i32,
  pub enableemail: i32,
  pub hidden: i32,
  pub emailoverride: String,
  pub keepnr: i32,
  pub checkinterval: i32,
  pub schedulingshares: i32,
  pub fetcherrormsg: Option<String>,
  pub forceeval: Option<bool>,
  pub starttime: Option<i32>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[table_name = "newsitems"]
pub struct NewsItem {
  pub id: i32,
  pub contents: String,
  pub createtime: i32,
  pub author: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(what)]
#[table_name = "nrbuilds"]
pub struct NrBuild {
  pub what: String,
  pub count: i32,
}

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(project, username)]
#[table_name = "projectmembers"]
pub struct ProjectMember {
  pub project: String,
  pub username: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(name)]
#[table_name = "projects"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Project {
  pub name: String,
  pub displayname: String,
  pub description: Option<String>,
  pub enabled: i32,
  pub hidden: i32,
  pub owner: String,
  pub homepage: Option<String>,
  pub declfile: Option<String>,
  pub decltype: Option<String>,
  pub declvalue: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, release_, build)]
#[table_name = "releasemembers"]
#[changeset_options(treat_none_as_null = "true")]
pub struct ReleaseMember {
  pub project: String,
  pub release_: String,
  pub build: i32,
  pub description: Option<String>,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(project, name)]
#[table_name = "releases"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Release {
  pub project: String,
  pub name: String,
  pub timestamp: i32,
  pub description: Option<String>,
}

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(username, project, jobset, job)]
#[table_name = "starredjobs"]
pub struct StarredJob {
  pub username: String,
  pub project: String,
  pub jobset: String,
  pub job: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(what)]
#[table_name = "systemstatus"]
pub struct SystemStatus {
  pub what: String,
  pub status: Value,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(system)]
#[table_name = "systemtypes"]
pub struct SystemType {
  pub system: String,
  pub maxconcurrent: i32,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(baseuri)]
#[table_name = "urirevmapper"]
pub struct UriRevMapper {
  pub baseuri: String,
  pub uri: String,
}

#[derive(QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable)]
#[primary_key(username, role)]
#[table_name = "userroles"]
pub struct UserRole {
  pub username: String,
  pub role: String,
}

#[derive(
  QueryableByName, Queryable, Insertable, Debug, Serialize, Deserialize, Identifiable, AsChangeset,
)]
#[primary_key(username)]
#[table_name = "users"]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
  pub username: String,
  pub fullname: Option<String>,
  pub emailaddress: String,
  pub password: String,
  pub emailonerror: i32,
  #[serde(rename = "type")]
  pub type_: String,
  pub publicdashboard: bool,
}
