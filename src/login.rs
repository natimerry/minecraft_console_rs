use password_lib::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
struct UserFromInput {
    user_name: String,
    password: String,
}

/// WEB PAGES

#[get("/")]
pub fn login_page() -> Template {
    Template::render(
        "index",
        context! {
            field: "Hello",
        },
    )
}

#[get("/register")]
pub async fn register() -> Template {
    Template::render("registered", context! {})
}

#[allow(private_interfaces)]
#[post("/login", data = "<account>")]
pub async fn login_auth(account: Form<UserFromInput>) -> Result<Template, Redirect> {
    let authentication_result =
        authenticate_with_password(&account.user_name, &account.password).await;

    match authentication_result {
        Ok(hashed_pass) => {
            let user_pass_hash = dbg!(format!("{}+mc+{}", &account.user_name, &hashed_pass));
            let hashed_pass = generate_sha512_string(user_pass_hash);
            return Ok(Template::render(
                "authed",
                context! {
                    user_name:&account.user_name,
                    key: hashed_pass
                },
            ));
        }
        Err(e) => {
            match e {
                Errors::PasswordFailure => (),
                Errors::NoSuchUser => {
                    password_lib::add_user(&account.user_name, &account.password).await.unwrap();
                    return Err(Redirect::moved("/register"));
                },
                _ => (),
            }
            return Err(Redirect::moved("/"));
        }
    }
}
