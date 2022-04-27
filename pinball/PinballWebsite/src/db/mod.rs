use std::env;
use std::sync::Arc;
use diesel::{Connection, NotFound, PgConnection, RunQueryDsl};
use diesel::result::Error;
use crate::db::Errors::{DbLoadErrRank, DbLoadErrUser};

use dotenv::dotenv;

pub mod models;
pub mod schema;
mod tests;

pub enum Errors{
    DbLoadErrRank,
    DbLoadErrUser,
    ConError,
    ConvertError
}


pub fn establish_connection()-> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error Connecting to {}", database_url))
}

pub fn query_scores() -> Vec<models::Score> {
    use crate::db::schema::scores::dsl::*;

    let connection = establish_connection();


    schema::scores::table
        .load::<models::Score>(&connection).expect("Error")
}

pub fn query_users() -> Vec<models::User>{
    use crate::db::schema::users::dsl::*;

    let connection = establish_connection();

    schema::users::table
        .load::<models::User>(&connection).expect("Error")
}