use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    command: String,

    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    #[arg(short, long, default_value_t = 8082 as u16)]
    port: u16,
}

use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    args: String,
}

#[get("/")]
async fn root(query: web::Query<Query>, command: web::Data<String>) -> impl Responder {
    let mut child = Command::new(command.to_string())
        .args(&query.args.split(' ').collect::<Vec<&str>>())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("failed to start `{:?}`", command.to_string()));
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let response_string = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(response_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = Args::parse();
    let command = args.command;
    let addr = args.addr;
    let port = args.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(command.clone()))
            .service(root)
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}
