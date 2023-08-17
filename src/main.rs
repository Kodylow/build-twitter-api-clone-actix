use actix_web::web::get;
use actix_web::HttpServer;

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| actix_web::App::new().route("/", get().to(index)))
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await;
}

async fn index() -> &'static str {
    "Hello World!"
}
