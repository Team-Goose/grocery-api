#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::account;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct Account {
    id: i32,
    username: String,
    pass: String,
    list: Vec<i32>,
    friends: Vec<i32>
}

#[derive(Insertable, Deserialize)]
#[table_name="account"]
struct NewAccount {
    username: String,
    pass: String,
}

// Tests connection
#[get("/test")]
fn test() -> String {
    String::from("Connection successful")
}

// Creates new user
#[post("/user/create", data="<new_account>")]
fn create_account(conn: DbConn, new_account: Json<NewAccount>) -> Json<Account> {
    let result = diesel::insert_into(account::table)
        .values(&new_account.0)
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

// Returns users as JSON
#[get("/account/all")]
fn get_all_accounts(conn: DbConn) -> Json<Vec<Account>>{
    Json(account::table
        .load::<Account>(&*conn)
        .unwrap())
}

#[get("/account/<id>")]
fn get_account(conn: DbConn, id: i32) -> Json<Account> {
    Json(account::table
        .filter(account::columns::id.eq(id))
        .get_result(&*conn)
        .unwrap())
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