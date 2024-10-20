/*
   Simple Status Page - a simple service status app built with rust
   Copyright (C) 2023-2024  Simon Stefan Barth

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published
   by the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use log::info;

use simple_status_page::endpoints::status;
use simple_status_page::setup_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = setup_app().expect("Failed to setup app");

    // Grab HTTP server configuration
    let host = settings.get_string("webserver.host").expect("Invalid host");
    let port = settings.get::<u16>("webserver.port").expect("Invalid port");
    let url = settings.get_string("webserver.url").expect("Invalid URL");

    info!("Listening on {}:{} as {}", host, port, url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&url)
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(status)
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
