mod dto;
mod settings;
mod repository;
mod model;
mod handlers;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_slog::StructuredLogger;
use slog::Drain;
use std::sync::Mutex;
use handlers::{HttpContext};
use settings::Settings;
use repository::Repository;
use handlers::{index, save, get, toggle_done_status, delete, get_all};

#[macro_use]
extern crate slog;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let logger = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    let settings = Settings::new(&logger).unwrap();
    let server_url = format!("0.0.0.0:{}",settings.server.port);
    info!(logger, "Listening on: {}", server_url);

    let http_context = HttpContext{
        logger,
        repository : Repository::new(&settings)
    };
    HttpServer::new(move || {
        App::new()
            .wrap(
                StructuredLogger::new(http_context.logger.new(o!("log_type" => "access")))
            )
            .wrap(Cors::permissive())
            .data(web::JsonConfig::default().limit(4096))
            .data(http_context.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/todos/{id}").route(web::delete().to(delete)).route(web::get().to(get)))
            .service(web::resource("/todos/{id}/done").route(web::patch().to(toggle_done_status)))
            .service(web::resource("/todos").route(web::post().to(save)).route(web::get().to(get_all)))
    }).bind(server_url)?.run().await
}
