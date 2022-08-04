#[macro_use] extern crate rocket;
use anyhow::Result;
use rocket::State;
// use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[get("/login/<id>/<pass>")]
async fn login(pool: &State<Pool<MySql>>, id: &str, pass: &str) -> String {
    let receiv_pass = sqlx::query!("SELECT pass FROM admin WHERE id = ?",id).fetch_one(&**pool).await.unwrap();
    if receiv_pass.pass == pass {
        "Login Success".to_string()
    } else {
        "Access Denied".to_string()
    }
}

#[get("/")]
async fn front() -> String {
    format!("Front page")
}

#[launch]
async fn run() -> _ {
    // et ldb = SqlitePoolOptions::new().connect("./data/data.db").await.unwrap();
    let db = MySqlPoolOptions::new().connect("mysql://root:@127.0.0.1/data").await.unwrap();
    rocket::build().mount("/", routes![login])
                   .mount("/", routes![front])
                   .manage(db)
}