#![feature(proc_macro_hygiene, decl_macro)]
mod db;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};
use crate::db::query_scores;


#[macro_use] extern crate rocket;

#[get("/ranks")]
fn ranks_get() -> String {
    let mut response: Vec<String> = vec![];

    let conn = establish_connection();
    for ranks in query_scores(&conn){
        response.push(ranks.)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ranks_get])
        .launch();
}