use actix_web::{ App, web, HttpServer, middleware};
use routes::handlers;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod routes{
    pub mod handlers;
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let  manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool= r2d2::Pool::builder()
        .build(manager)
        .expect("FAILED TO CREATE POOL");

    HttpServer::new( move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .service(handlers::health_check)
        .service(handlers::index)
        .service(handlers::add_business)
        .service(handlers::get_businesses)
        .service(handlers::get_single_business)
        .service(handlers::update_business)
        .service(handlers::delete_business)
        .service(handlers::add_review)
        .service(handlers::get_reviews)
        .service(handlers::get_single_review)
        .service(handlers::update_review)
        .service(handlers::delete_review)
        .service(handlers::add_user)
        .service(handlers::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
