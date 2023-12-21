use std::{cell::LazyCell, sync::Mutex};
use password_lib::generate_sha512_string;
use rocket::{self, get, form::FromForm, response::Redirect, post, data, };
use rocket_dyn_templates::{context, Template};

use rocket::form::Form;
use sha2::{Digest, Sha512};

static LIST_OF_USERS: Mutex<LazyCell<Vec<String>>> = Mutex::new(LazyCell::new(||{
    std::fs::read_to_string("password_list.txt").unwrap().lines().map(|line|{
        if let Some((user,pass)) = line.split_once(":"){
            let user_pass = dbg!(format!("{}+mc+{}", user, pass));
            generate_sha512_string(user_pass)

        } else {
            panic!("UNABLE TO LAZILY EVALUATE LIST OF USERS");
        }
    }).collect()
}));


#[derive(FromForm,Debug)]
struct AuthedUser{
    user_name: String,
    key: String,
}

#[allow(private_interfaces)]
#[post("/landing",data = "<authed_user>")]
pub fn landing_page(authed_user: Form<AuthedUser>) -> Result<Template,Redirect> {
    let mut flag = false;
    LIST_OF_USERS.lock().unwrap().iter().for_each(|user|{
        if user == &authed_user.key{
            flag = true;
        }
    });
    if flag{
        return Ok(Template::render("landing_page", context! {user:authed_user.user_name.clone()}));
    }
    Err(Redirect::moved("/"))
}
