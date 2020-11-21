#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;
mod account;

use crate::account::*;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::databases::diesel::Connection;
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use diesel_migrations::*;
use serde_derive::{Serialize, Deserialize};
// use crate::account::serde_json::read::error::string::fmt::run;


// Tests connection
#[get("/test")]
fn test() -> String {
    String::from("Connection successful")
}

// #[post("/list/add")]

// Returns list attached to user account
// #[get("/account/<id>/list")]
// fn account_list(conn: DbConn, id: i32) -> Json<Account::list> {
//     Json(account::table
//         .filter(account::columns::id.eq(id))
//         .get_result(&*conn)
        
//         .unwrap())
// }

embed_migrations!("../migrations");

fn main() {
    let db_conn = PgConnection::establish("postgres://rocket:rocket@localhost/rocket")
        .expect(&format!("Error connecting to {}", "postgres://rocket:rocket@localhost/rocket"));

    embedded_migrations::run(&db_conn);    

    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            test,
            login,
            create_account,
            get_all_accounts,
            add_to_list,
            remove_from_list,
            add_friend,
            remove_friend,
        ]).launch();
}