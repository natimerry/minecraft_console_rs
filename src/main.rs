#![feature(lazy_cell)]

mod landing;
mod login;


use crate::landing::*;
use landing::landing_page;
use login::{login_auth, login_page};
use rocket::{
    self,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;
use std::env;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![login_page])
        .mount("/", routes![login_auth])
        .mount("/", routes![landing_page])
        .mount("/", routes![rx_channel])
        .mount("/", routes![tx_channel])
}
