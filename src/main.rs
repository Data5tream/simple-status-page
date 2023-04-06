use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::cache::load_config;
use crate::endpoints::status;
use crate::watcher::setup_watcher;
use simple_status_page::get_config;

mod cache;
mod endpoints;
mod watcher;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_config();

    // Load config file into cache, exit if no watchpoints are configured
    let has_watchpoints = load_config();
    if !has_watchpoints {
        println!("No watchpoints configured! See README.md for instructions");
        return Ok(());
    }

    // Create watcher thread
    setup_watcher();

    // Grab HTTP server configuration
    let host = settings.get_string("webserver.host").expect("Invalid host");
    let port = settings.get::<u16>("webserver.port").expect("Invalid port");
    let url = settings.get_string("webserver.url").expect("Invalid URL");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&url)
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new().wrap(cors).service(status)
    })
    .bind((host, port))?
    .run()
    .await
}
