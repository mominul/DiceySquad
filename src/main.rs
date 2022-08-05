

use anyhow::Result;
use rocket::{State, form::FromForm, tokio::fs::read_to_string};
// use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use rocket::{get, post, routes, launch};
use rocket::form::Form;

#[derive(FromForm)]
struct LogIn<'r> {
    id: &'r str,
    pass: &'r str,
}

#[post("/", data = "<login>")]
async fn login(pool: &State<Pool<MySql>>, login: Form<LogIn<'_>>) -> String {
    let receiv_pass = sqlx::query!("SELECT pass FROM admin WHERE id = ?",login.id).fetch_one(&**pool).await.unwrap();
    if receiv_pass.pass == login.pass {
        "Login Success".to_string()
    }
    else {
        "Access Denied".to_string()
    }
}

#[get("/")]
async fn front() -> rocket::response::content::RawHtml<String> {
    let html = read_to_string("./template/sample.html").await.unwrap();
    rocket::response::content::RawHtml(html)
}

#[launch]
async fn run() -> _ {
    // et ldb = SqlitePoolOptions::new().connect("./data/data.db").await.unwrap();
    let db = MySqlPoolOptions::new().connect("mysql://root:@127.0.0.1/data").await.unwrap();
    rocket::build().mount("/", routes![login])
                   .mount("/", routes![front])
                   .manage(db)
}