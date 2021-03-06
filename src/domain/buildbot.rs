use chrono::NaiveDateTime;
use diesel::prelude::*;
use super::schema::*;

#[derive(AsChangeset, Clone, Debug, Deserialize, Eq, Insertable,
         Ord, PartialEq, PartialOrd, Queryable, Serialize)]
#[table_name="build"]
pub struct Build {
    pub number: i32,
    pub builder_name: String,
    pub successful: bool,
    pub message: String,
    pub duration_secs: Option<i32>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}
