use crate::schema::account;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};
use serde_json;

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

// Logs in with username and password, returns user id
#[get("/login/<username>/<pass>")]
pub fn login(conn: DbConn, username: String, pass: String) -> Json<Account> {
     let results: Vec<Account> = account::table
        .filter(account::columns::username.eq(username))
        .get_results(&*conn)
        .unwrap();

    let mut out = Json(Account {
        id: -1,
        username: String::from("ERROR"),
        pass: String::from("ERROR"),
        list: String::from("ERROR"),
        friends: String::from("ERROR"),
        isAdmin: false
    });
    
    for a in results {
        if a.pass.eq(&pass) {
            out = Json(a);
        };
    };

    out
}

// Creates new account
#[post("/account/create", data="<new_account>")]
pub fn create_account(conn: DbConn, new_account: Json<NewAccount>) -> Json<Account> {
    Json(diesel::insert_into(account::table)
        .values(&new_account.0)
        .get_result(&*conn)
        .unwrap())
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

// Adds an item to the list of the user id
#[get("/list/add/<id>/<product>")]
pub fn add_to_list(conn: DbConn, id: i32, product: i32) -> bool {
    let acct = account::table
        .filter(account::columns::id.eq(id))
        .get_result(&*conn);

    let a: Account = acct.unwrap();

    let map: Vec<i32> = serde_json::from_str(&a.list).expect("deserialize");
    map.push(product);
    let l = serde_json::to_string(&map).unwrap();

    diesel::update(
        account::table.filter(
            account::columns::id.eq(id)
        )).set(account::columns::list.eq(l))
        .get_result(&*conn).unwrap();

    true
}