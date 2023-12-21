use std::fs;

use sha2::{Digest,Sha512};


pub fn generate_sha512_string(string:String) -> String{
    let mut hasher = Sha512::new();
    hasher.update(string.as_bytes());
    let result = hasher.clone().finalize();
    format!("{:x}",result) 
}

#[derive(Debug)]
pub enum Errors {
    PasswordFailure,
    NoSuchUser,
    BadRequest,
    InternalServerError,
}

pub async fn authenticate_with_password(user: &str, pass: &str) -> Result<String, Errors> {
    let mut hasher = Sha512::new();
    let password_data = fs::read_to_string("./password_list.txt");

    match password_data {
        Err(_) => return Err(Errors::InternalServerError),
        _ => (),
    }

    let binding = password_data.unwrap();
    let lines = binding.lines();

    for line in lines {
        if let Some((file_user, stored_pass)) = line.split_once(':') {
            hasher.update(pass.as_bytes());
            let result = hasher.clone().finalize();
            let hashed_pass = format!("{:x}", result);

            if file_user == user && stored_pass == hashed_pass {
                return Ok(hashed_pass);
            } else if file_user == user && stored_pass != hashed_pass {
                return Err(Errors::PasswordFailure);
            }
        }
    }
    return Err(Errors::NoSuchUser);
}

// async fn check_if_authed(key:String) -> {
    
// }