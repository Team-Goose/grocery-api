use crate::schema::account;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
pub struct Account {
    id: i32,
    username: String,
    pass: String,
    list: String, // these are stored as serialized json lists
    friends: String, // when editing these lists you will need to unserialized the lists, edit them, then reserialize them before putting them in the database
    isAdmin: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="account"]
pub struct NewAccount {
    username: String,
    pass: String,
}

// Creates new account
#[post("/account/create", data="<new_account>")]
pub fn create_account(conn: DbConn, new_account: Json<NewAccount>) -> Json<Account> {
    let result = diesel::insert_into(account::table)
        .values(&new_account.0)
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

// Returns users as JSON
#[get("/account/all")]
pub fn get_all_accounts(conn: DbConn) -> Json<Vec<Account>>{
    Json(account::table
        .load::<Account>(&*conn)
        .unwrap())
}

#[get("/account/<id>")]
pub fn get_account(conn: DbConn, id: i32) -> Json<Account> {
    Json(account::table
        .filter(account::columns::id.eq(id))
        .get_result(&*conn)
        .unwrap())
}