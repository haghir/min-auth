use actix_web::{
    http::{header}, web::{Data, Path, Form},
    get, post, error,
    App, HttpServer, HttpResponse, Error
};
use std::{sync::Mutex, collections::HashMap};
use tera::{Tera, Context};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let templates = Data::new(Mutex::new(Tera::new("html/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&storage))
            .service(index)
            .service(show)
            .service(new)
            .service(do_new)
            .service(edit)
            .service(do_edit)
            .service(delete)
            .service(do_delete)
            .service(panic)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
