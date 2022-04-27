#![feature(proc_macro_hygiene, decl_macro)]
mod db;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use std::env;
use std::fmt::format;
use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use crate::db::{establish_connection, query_scores};


#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;

#[get("/ranks")]
fn ranks_get() -> String {
    let mut response: Vec<String> = vec![];

    for ranks in query_scores(){
        response.push(format!("User Id: {}",ranks.get_id()));
    }

    response.join("\n")
}
#[get("/add")]
fn add_get() -> String {
    "Response added".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ranks_get,add_get])
        .launch();
}