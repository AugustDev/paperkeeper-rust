#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod controllers;

pub fn establish_connection() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

pub fn start() {
    println!("Hello, world!");
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/users", routes![controllers::users::get_user])
        .launch();
    establish_connection();
}
