mod json_serializaitions;
mod process;
mod state;
mod to_do;
mod views;

use std::{env, vec::Vec};
use views::views_factory;

use actix_web::{App, HttpServer};
use async_std::io;
use process::process_input;
use state::read_file;
use to_do::to_do_factory;

#[actix_rt::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        command_line_interface(&args);
    } else {
        let result = run_server().await;
        match result {
            Ok(_) => println!("Server successfully ran."),
            Err(err) => panic!("{}", err),
        }
    }
}

fn command_line_interface(args: &[String]) {
    let command = &args[1];

    let title: &String = &args[2];

    let state = read_file("state.json");

    let status: String = match state.get(title) {
        Some(json_val) => json_val.to_string().replace('\"', ""),
        None => String::from("pending"),
    };

    let item = match to_do_factory(&status, title) {
        Some(item) => item,
        None => panic!("{}", "Invalid status provided."),
    };

    // process input here
    process_input(item, command.clone(), &state);
}

async fn run_server() -> io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(|| App::new().configure(views_factory))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
