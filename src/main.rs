#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::fs::{FileServer, NamedFile};
use std::path::Path;

mod error;


const  SLEEP_SERVER_TIME: u8 = 0;


#[get("/")]
async fn index() -> Option<NamedFile>  {
    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    NamedFile::open(Path::new("static/index.html")).await.ok()
    
}

#[get("/register")]
async fn register() -> Option<NamedFile>  {
    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    NamedFile::open(Path::new("static/register.html")).await.ok()
}
#[get("/login")]
async fn login() -> Option<NamedFile>  {
    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    NamedFile::open(Path::new("static/login.html")).await.ok()
}
#[launch]
async fn rocket() -> _ {
    error::error_tester();
    rocket::build()
    .mount("/", routes![index])
    .mount("/", FileServer::from("static"))
    .mount("/", routes![register])
    .mount("/", routes![login])

}