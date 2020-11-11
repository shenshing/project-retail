use actix_web::{HttpServer, App, web, middleware};

use video_upload::video::{save_file, index};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file)),
        )
    })
    .bind(ip)?
    .run()
    .await
}
