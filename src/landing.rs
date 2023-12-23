use core::time;
use password_lib::generate_sha512_string;
use rocket::{
    form::FromForm,
    futures::{SinkExt, StreamExt},
    get, post,
    response::Redirect,
   
};
use rocket_dyn_templates::{context, Template};
use std::{
    cell::LazyCell,
    sync::Mutex,
};

use rocket::form::Form;

use std::io::Write;
static LIST_OF_USERS: Mutex<LazyCell<Vec<String>>> = Mutex::new(LazyCell::new(|| {
    std::fs::read_to_string("password_list.txt")
        .unwrap()
        .lines()
        .map(|line| {
            if let Some((user, pass)) = line.split_once(":") {
                let user_pass = dbg!(format!("{}+mc+{}", user, pass));
                generate_sha512_string(user_pass)
            } else {
                panic!("UNABLE TO LAZILY EVALUATE LIST OF USERS");
            }
        })
        .collect()
}));

#[derive(FromForm, Debug)]
struct AuthedUser {
    user_name: String,
    key: String,
}

#[allow(private_interfaces)]
#[post("/landing", data = "<authed_user>")]
pub fn landing_page(authed_user: Form<AuthedUser>) -> Result<Template, Redirect> {
    let mut flag = false;
    LIST_OF_USERS.lock().unwrap().iter().for_each(|user| {
        if user == &authed_user.key {
            flag = true;
        }
    });
    if flag {
        return Ok(Template::render(
            "landing_page",
            context! {user:authed_user.user_name.clone()},
        ));
    }
    Err(Redirect::moved("/"))
}

// only process commands from this channel
#[get("/tx", rank = 1)]
pub fn rx_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    // This is entirely optional. Change default configuration.
    let ws = ws.config(ws::Config {
        // set max message size to 3MiB
        max_message_size: Some(3 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                // stream.send("lolbro".into()).await?;
                // Receive some messsage from the server.
                match message {
                    Ok(_) => {
                        let mut file: std::fs::File = std::fs::OpenOptions::new()
                        .write(true)
                        .open("./input_fifo")
                        .unwrap();
                        let _ = write!(file,"{}\n", dbg!(message.unwrap()));

                    },
                    Err(e) => {
                        println!("Error in awaiting next stream message from the WebSocket");
                        println!("[WebSocket] Error: {:?}", e);
                    }
                }

       
            }

            Ok(())
        })
    })
}

#[get("/rx", rank = 1)]
pub fn tx_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    let ws = ws.config(ws::Config {
        // set max message size to 3MiB
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let file = rocket::tokio::fs::read_to_string("./server_output").await?;
                let converted = ansi_to_html::convert(&file).unwrap();
                
                let x = stream.send(converted.clone().into()).await;
                match x{
                    Ok(_) => (),
                    Err(_) => println!("Error in sending data"),
                }

                // println!("{}", file);

                std::thread::sleep(time::Duration::from_millis(100));
            }
        })
    })
}
