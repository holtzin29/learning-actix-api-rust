use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};

mod utils;
mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
   if std::env::var_os("RUST LOG").is_none() {
       unsafe {
           std::env::set_var("RUST_LOG", "actix_web=info");
       }
   }
   dotenv::dotenv().ok();
   env_logger::init();
   
   let port: u16 = (*utils::constants::PORT).clone();
   let address: String = (*utils::constants::ADDRESS).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(|config: &mut ServiceConfig| routes::home_routes::config(config))
    })
    .bind((address, port))?.run().await


}
