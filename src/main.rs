use std::sync::Mutex;

use actix_web::web::{get, post, Data};
use actix_web::{App, HttpServer};

#[derive(Clone)]
struct Messenger {
    message: String,
}

#[derive(Clone)]
struct AppState {
    messenger: Messenger,
}

#[actix_web::main]
async fn main() {
    println!("Server is running on port 8000");
    let app_state = AppState {
        messenger: Messenger {
            message: "Hello".to_string(),
        },
    };

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .route("/", post().to(update))
            .route("/", get().to(index))
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run()
    .await;
}

async fn update(app_data: actix_web::web::Data<AppState>) -> String {
    let messenger = app_data.messenger.clone();
    format!("{} World!", messenger.message)
}

async fn index(app_data: actix_web::web::Data<AppState>) -> String {
    app_data.messenger.message.clone()
}
