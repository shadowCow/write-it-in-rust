use actix_web::{web, App, HttpServer};

use rust_game_ai::analysis_server::{
    find_best_action,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8383;
    let address = format!("127.0.0.1:{:?}", port);

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/shut-the-box")
                    .route("/find-best-action", web::get().to(find_best_action))
            )
    })
    .bind(address.to_owned())?
    .run()
    .await
}
