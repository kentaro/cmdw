use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tinytemplate::TinyTemplate;

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

#[derive(Deserialize)]
struct Query {
    args: Option<String>,
}

#[derive(Serialize)]
struct Context {
    command: String,
    args: String,
}

#[get("/")]
async fn index(
    query: web::Query<Query>,
    tmpl: web::Data<TinyTemplate<'_>>,
    command: web::Data<String>,
) -> impl Responder {
    let args = match query.args.clone() {
        Some(args) => args,
        None => "".to_string(),
    };
    let ctx = Context {
        command: command.to_string(),
        args: args,
    };
    let response_string = {
        tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
            .unwrap()
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_string)
}

#[get("/command")]
async fn root(query: web::Query<Query>, command: web::Data<String>) -> impl Responder {
    let response_string = match query.args.clone() {
        Some(args) => {
            let mut child = Command::new(command.to_string())
                .args(args.split(' ').collect::<Vec<&str>>())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("failed to start `{:?}`", command.to_string()));
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            reader
                .lines()
                .map(|line| line.unwrap())
                .collect::<Vec<String>>()
                .join("\n")
        }
        None => "".to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(response_string)
}

static INDEX: &str = include_str!("../index.html");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let command = args.command;
    let addr = args.addr;
    let port = args.port;

    HttpServer::new(move || {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(tt))
            .app_data(web::Data::new(command.clone()))
            .service(index)
            .service(root)
    })
    .bind(format!("{addr}:{port}"))?
    .run()
    .await
}
