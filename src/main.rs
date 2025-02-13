use std::{fs::File, io::BufReader};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use validator::{Validate, ValidationError};


async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

// Struct for POST request data
#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Struct for response data
#[derive(Deserialize, Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct RJLocation {
    address: Option<String>,
    postalCode: Option<String>,
    city: Option<String>,
    countryCode: String,
    region: Option<String>
}

#[derive(Deserialize, Serialize, Validate)]
struct RJBasics {
    name: String,
    label: String,
    image: String,
    #[validate(email)]
    email: String,
    phone: String,
    url: String,
    summary: String,
    location: RJLocation
}

// JSON Resume Base
#[derive(Deserialize, Serialize, Validate)]
struct ResumeJson {
    #[serde(rename = "$schema")]
    schema: String,
    #[validate(nested)]
    basics: RJBasics,
    // work: String,
    // volunteer: String,
    // education: String,
    // awards: String,
    // certificates: String,
    // publications: String,
    // skills: String,
    // languages: String,
    // interests: String,
    // references: String,
    // projects: String
}



// #[derive(Deserialize, Serialize)]
// struct ContactInfo {
//     phone: String,
//     email: String,
//     address: Address,
// }

// #[derive(Deserialize, Serialize)]
// struct Project {
//     id: u32,
//     name: String,
//     description: String,
//     start_date: String,
//     end_date: Option<String>,
//     status: String,
// }

// #[derive(Deserialize, Serialize)]
// struct ComplexUser {
//     id: u32,
//     username: String,
//     first_name: String,
//     last_name: String,
//     contact: ContactInfo,
//     projects: Vec<Project>,
//     created_at: u64,
//     updated_at: u64,
//     is_active: bool,
// }



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

fn option_to_string(option: &Option<String>) -> String {
    match &option {
        Some(value) => value.clone(),
        None => "".to_string(), // Directly returning an empty string slice
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
fn main() -> Result<(), ValidationError> {
    println!("Server starting at http://127.0.0.1:8080");

    let start_time = get_current_timestamp();
    println!("This time! {}", start_time);

    
    // tokio::spawn(async move {
    //     loop {
    //         let current_time = get_current_timestamp();
    //         println!("Heartbeat: Server has been running for {} seconds", current_time-start_time);
    //         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    //     }
    // });

    match File::open("resume.yaml") {
        Ok(file) => {
            let result = serde_yaml::from_reader::<_, ResumeJson>(file);
            match result {
                Ok(data) => {
                    // let acity: String = option_to_string(&data.basics.location.city);
                    let acity_raw = data.basics.location.city.clone();
                    let acity: String = acity_raw.unwrap_or("".to_string());
                    println!("{}", acity);
                    println!("{}", data.basics.email);
                    match data.validate() {
                        Ok(_) => {
                            println!("yaml validated!");
                            Ok(())

                        },
                        Err(e) => {
                            eprintln!("Failed to parse YAML: {:?}", e);
                            return Err(ValidationError::new("Validation  failed"));  // Return a ValidationError on deserialization failure
                        }
                    }
                }
                Err(e) => return Err(ValidationError::new("Deserialization  failed"))
            }
        },
        Err(e) => return Err(ValidationError::new("File IO failed"))
    }

    // HttpServer::new(|| {
    //     App::new()
    //         .service(
    //             web::resource("/users/{user_id}")
    //                 .route(web::get().to(get_user))
    //                 .route(web::put().to(update_user))
    //                 .route(web::delete().to(delete_user))
    //         )
    //         .service(
    //             web::resource("/users")
    //                 .route(web::post().to(create_user))
    //         )
    //         .service(
    //             web::resource("/test")
    //                 .route(web::get().to(show_raw_data))
    //         )
    //         .service(
    //             web::resource("/hello/{name}")
    //                 .route(web::get().to(greet))
    //         )
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_create_user() {
        // Create test app
        let app = test::init_service(
            App::new().service(web::resource("/users").route(web::post().to(create_user)))
        ).await;

        // Create test payload
        let payload = CreateUser {
            name: String::from("Test User"),
            email: String::from("test@example.com"),
        };

        // Create test request
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&payload)
            .to_request();

        // Perform test request
        let resp = test::call_service(&app, req).await;

        // Assert response status
        assert!(resp.status().is_success());

        // Parse response body
        let body: UserResponse = test::read_body_json(resp).await;

        // Assert response contents
        assert_eq!(body.name, "Test User");
        assert_eq!(body.email, "test@example.com");
        assert_eq!(body.message, "User created successfully");
    }
}