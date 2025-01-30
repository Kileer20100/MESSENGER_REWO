#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::fs::{FileServer, NamedFile};
use std::path::Path;

const  SLEEP_SERVER_TIME: u8 = 0;


#[get("/")]
async fn index() -> Option<NamedFile>  {
    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    
    NamedFile::open(Path::new("static/index.html")).await.ok()
}
#[get("/world/<seconds>")]           
async fn world(seconds:u16) -> String {  

    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    format!("seka is {}",seconds)
}
#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![world])
    .mount("/", FileServer::from("static"))
}
