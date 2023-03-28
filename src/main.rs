use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::Config;

use crate::watcher::Watchpoint;

mod watcher;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect("Invalid or missing config file");

    let raw_watchlist = settings.get_array("watcher.watchlist").expect("Invalid watch list");

    // Make sure we have something to watch
    if raw_watchlist.is_empty() {
        println!("No entries in watchlist! Nothing to do");
        return Ok(());
    }

    let mut watchlist: Vec<Watchpoint> = Vec::new();
    for i in &raw_watchlist {
        watchlist.push(i.clone().try_deserialize().expect("invalid config value"));
    }

    let interval = settings.get::<u32>("watcher.interval").expect("Invalid interval");

    actix_rt::spawn(async move {
        watcher::start_watcher(interval, &watchlist).await;
    });

    // Grab HTTP server configuration
    let host = settings.get_string("webserver.host").expect("Invalid host");
    let port = settings.get::<u16>("webserver.port").expect("Invalid port");

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind((host, port))?
        .run()
        .await
}
