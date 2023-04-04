use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::Config;
use redis::Commands;

use crate::cache::get_redis_connection;
use crate::watcher::{Watchpoint, WatchpointStatus};

mod cache;
mod watcher;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/status")]
async fn status() -> impl Responder {
    let mut con = get_redis_connection();
    let watchpoints: Vec<Watchpoint> = match con.get::<&str, String>("config:ids") {
        Ok(k) => serde_json::from_str(k.as_str()).unwrap(),
        Err(_e) => return HttpResponse::InternalServerError().finish(),
    };

    let mut data: Vec<WatchpointStatus> = Vec::new();
    for watchpoint in watchpoints {
        let status = con
            .get::<&str, u16>(format!("status:{}:status-code", watchpoint.id).as_str())
            .unwrap();

        data.push(WatchpointStatus { watchpoint, status });
    }

    HttpResponse::Ok().json(data)
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

    let raw_watchlist = settings
        .get_array("watcher.watchlist")
        .expect("Invalid watch list");

    // Make sure we have something to watch
    if raw_watchlist.is_empty() {
        println!("No entries in watchlist! Nothing to do");
        return Ok(());
    }

    let mut watchlist: Vec<Watchpoint> = Vec::new();
    for i in &raw_watchlist {
        watchlist.push(i.clone().try_deserialize().expect("invalid config value"));
    }

    let mut con = get_redis_connection();
    let _: () = con
        .set("config:ids", serde_json::to_string(&watchlist).unwrap())
        .unwrap();

    let interval = settings
        .get::<u32>("watcher.interval")
        .expect("Invalid interval");

    actix_rt::spawn(async move {
        watcher::start_watcher(interval, &watchlist).await;
    });

    // Grab HTTP server configuration
    let host = settings.get_string("webserver.host").expect("Invalid host");
    let port = settings.get::<u16>("webserver.port").expect("Invalid port");

    HttpServer::new(|| {
        let cors = Cors::permissive()
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new().wrap(cors).service(status).service(hello)
    })
    .bind((host, port))?
    .run()
    .await
}
