#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::fs::{FileServer, NamedFile};
use std::path::Path;
use rocket::form::Form;

mod error;
mod auth_validation;

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





#[derive(FromForm)]
struct RegisterForm {
    username: String,
    password: String,
    #[field(name = "confirm-password")]
    confirm_password: String,
    
}

#[post("/submit", data = "<form>")]
async fn registerform(form: Form<RegisterForm>) -> String {



    let _call_tester_reg = auth_validation::validate_aut(form.username.to_string(), form.password.to_string(), form.confirm_password.to_string(), 2);
    if _call_tester_reg == true {
        format!("{} {} {}", form.username, form.password, form.confirm_password)
    }
    else {
        format!("Fail!")
    }
    
}

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/log", data = "<form>")]
fn loginfofrm(form: Form<LoginForm>) -> String {
    let _call_tester_log =auth_validation::validate_aut(form.username.to_string(), form.password.to_string(), "0".to_string(), 1);
    format!("{} {} ", form.username, form.password)
}

#[get("/list/network")]
async fn list_network() -> Option<NamedFile> {
    sleep(Duration::from_secs(SLEEP_SERVER_TIME.into())).await;
    NamedFile::open(Path::new("messanger/serverlist.html")).await.ok()
}


#[launch]
async fn rocket() -> _ {
    error::error_tester();
    
    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from("static"))
    .mount("/", routes![register])
    .mount("/", routes![login])
    .mount("/", routes![registerform])
    .mount("/", routes![loginfofrm])
    .mount("/messanger", FileServer::from("messanger"))
    .mount("/", routes![list_network])

}