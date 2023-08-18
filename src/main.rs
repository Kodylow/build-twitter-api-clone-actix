use std::sync::{Arc, Mutex, RwLock};

use actix_web::web::{self, get, post, Data, Json};
use actix_web::{App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct EntityId {
    id: i64,
}

#[derive(Clone)]
struct FinalUser {
    id: i64,
    user_name: String,
    full_name: String,
}

#[derive(Deserialize)]
struct NewUser {
    user_name: String,
    full_name: String,
}

// RwLock is a faster form of mutex because it allows multiple readers to acquire the lock at the same time.
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<FinalUser>>>,
}

#[actix_web::main]
async fn main() {
    println!("Server is running on port 8000");
    let app_state = AppState {
        users: Arc::new(RwLock::new(vec![
            FinalUser {
                id: 1,
                user_name: "user1".to_string(),
                full_name: "User One".to_string(),
            },
            FinalUser {
                id: 2,
                user_name: "user2".to_string(),
                full_name: "User Two".to_string(),
            },
            FinalUser {
                id: 3,
                user_name: "user3".to_string(),
                full_name: "User Three".to_string(),
            },
        ])),
    };

    let _ = HttpServer::new(move || {
        App::new().app_data(Data::new(app_state.clone())).service(
            web::scope("/v1")
                .service(web::resource("/users/{id}").route(web::get().to(get_user_name)))
                .service(web::resource("/user").route(web::post().to(insert_user))),
        )
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run()
    .await;
}

async fn insert_user(app_data: web::Data<AppState>, new_user: Json<NewUser>) -> String {
    let mut users = app_data.users.write().unwrap();
    let user = FinalUser {
        id: users.len() as i64 + 1,
        user_name: new_user.user_name.clone(),
        full_name: new_user.full_name.clone(),
    };
    users.push(user);
    format!("User {} inserted", new_user.user_name)
}
async fn get_user_name(app_data: web::Data<AppState>, params: web::Path<EntityId>) -> String {
    let users = app_data.users.read().unwrap();
    let user = users.iter().find(|&x| x.id == params.id);
    match user {
        Some(user) => user.user_name.clone(),
        None => "User not found".to_string(),
    }
}
