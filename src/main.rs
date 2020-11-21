#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;
mod account;

use crate::account::*;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};


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

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            test,
            create_account,
            get_all_accounts,
            // account_list,
            
        ]).launch();
}