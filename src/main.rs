#![feature(lazy_cell)]

use password_lib;
mod landing;
mod login;

use landing::landing_page;
use rocket::{
    self,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;

use login::{login_auth, login_page};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![login_page])
        .mount("/", routes![login_auth])
        .mount("/", routes![landing_page])
}
