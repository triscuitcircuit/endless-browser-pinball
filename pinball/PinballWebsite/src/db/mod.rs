use std::sync::Arc;
use diesel::{NotFound, PgConnection, RunQueryDsl};
use diesel::result::Error;
use crate::db::Errors::{DbLoadErrRank, DbLoadErrUser};

pub mod models;
pub mod schema;

pub enum Errors{
    DbLoadErrRank,
    DbLoadErrUser,
    ConError,
    ConvertError
}

pub fn query_scores(connection: &PgConnection ) -> Vec<models::Score> {
    schema::scores::table
        .load::<models::Score>(connection).expect("Error")
}

pub fn query_users(connection: &PgConnection) -> Vec<models::User>{
    schema::users::table
        .load::<models::User>(connection).expect("Error")
}