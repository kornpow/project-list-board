use std::{fs::File, io::BufReader};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};



async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

// Struct for POST request data
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Struct for response data
#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
    message: String,
}

// GET handler
async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    let user = UserResponse {
        id: *user_id,
        name: String::from("Example User"),
        email: String::from("user@example.com"),
        message: format!("Retrieved user {}", user_id),
    };
    
    HttpResponse::Ok().json(user)
}

// POST handler
async fn create_user(user: web::Json<CreateUser>) -> impl Responder {
    let response = UserResponse {
        id: 1,
        name: user.name.clone(),
        email: user.email.clone(),
        message: String::from("User created successfully"),
    };
    
    HttpResponse::Created().json(response)
}

// PUT handler
async fn update_user(
    user_id: web::Path<u32>,
    user: web::Json<CreateUser>
) -> impl Responder {
    let response = UserResponse {
        id: *user_id,
        name: user.name.clone(),
        email: user.email.clone(),
        message: format!("User {} updated successfully", user_id),
    };
    
    HttpResponse::Ok().json(response)
}

// DELETE handler
async fn delete_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id)
    }))
}

async fn show_raw_data() -> impl Responder {
    match File::open("data.json") {
        Ok(file) => {
            let result = serde_json::from_reader::<_, serde_json::Value>(file);
            match result {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::InternalServerError().json(format!("Failed to parse JSON: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to open file: {}", e))
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");

    let start_time = get_current_timestamp();
    println!("This time! {}", start_time);

    
    tokio::spawn(async move {
        loop {
            let current_time = get_current_timestamp();
            println!("Heartbeat: Server has been running for {} seconds", current_time-start_time);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/users/{user_id}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            )
            .service(
                web::resource("/users")
                    .route(web::post().to(create_user))
            )
            .service(
                web::resource("/test")
                    .route(web::get().to(show_raw_data))
            )
            .service(
                web::resource("/hello/{name}")
                    .route(web::get().to(greet))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}