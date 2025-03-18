fn main() {
    println!("Hello, world!");
}


// use actix_web::{web, App, HttpServer, HttpResponse, middleware};
// use mongodb::{Client, options::ClientOptions, Collection};
// use serde::{Serialize, Deserialize};
// use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
// use bcrypt::{hash, verify, DEFAULT_COST};
// use chrono::{Utc, Duration};
// use rand::Rng;
// use std::env;

// #[derive(Clone)]
// struct AppState {
//     db: Collection<User>,
//     jwt_secret: String,
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct User {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     id: Option<mongodb::bson::oid::ObjectId>,
//     name: String,
//     email: String,
//     username: String,
//     password: String,
//     profile_pic: Option<String>,
//     phone: String,
//     date_of_birth: chrono::DateTime<Utc>,
//     gender: String,
//     address: String,
//     city: String,
//     state: String,
//     country: String,
//     zip_code: String,
//     role: String,
//     created_at: chrono::DateTime<Utc>,
//     updated_at: chrono::DateTime<Utc>,
//     is_active: bool,
//     preferences: Option<std::collections::HashMap<String, String>>,
//     behavior_score: Option<f64>,
//     last_prediction: Option<chrono::DateTime<Utc>>,
// }

// #[derive(Serialize)]
// struct ErrorResponse {
//     error: String,
// }

// #[derive(Serialize)]
// struct SuccessResponse<T> {
//     message: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     data: Option<T>,
// }

// #[derive(Deserialize)]
// struct LoginRequest {
//     email: String,
//     password: String,
// }

// #[derive(Serialize)]
// struct Claims {
//     email: String,
//     role: String,
//     exp: usize,
//     iat: usize,
// }

// async fn setup_database() -> Result<Collection<User>, Box<dyn std::error::Error>> {
//     let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb+srv://machinelearner646:S2WJjm80GcgaqMiV@cluster0.aiigs.mongodb.net/goapp?retryWrites=true&w=majority".to_string());
//     let mut client_options = ClientOptions::parse(&mongo_uri).await?;
//     client_options.app_name = Some("RustApp".to_string());
//     let client = Client::with_options(client_options)?;
//     let db = client.database("goapp");
//     Ok(db.collection("users"))
// }

// async fn register(
//     user: web::Json<User>,
//     state: web::Data<AppState>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let mut new_user = user.into_inner();
    
//     // Check if user exists
//     let existing = state.db
//         .find_one(doc! {"$or": [{"email": &new_user.email}, {"username": &new_user.username}]}, None)
//         .await?;
    
//     if existing.is_some() {
//         return Ok(HttpResponse::Conflict().json(ErrorResponse {
//             error: "Email or username already exists".to_string(),
//         }));
//     }

//     // Hash password
//     new_user.password = hash(&new_user.password, DEFAULT_COST)?;
//     new_user.created_at = Utc::now();
//     new_user.updated_at = Utc::now();
//     new_user.role = "user".to_string();
//     new_user.is_active = true;

//     let result = state.db.insert_one(&new_user, None).await?;
//     let token = generate_token(&new_user, &state.jwt_secret)?;

//     Ok(HttpResponse::Created().json(SuccessResponse {
//         message: "Registration successful".to_string(),
//         data: Some(serde_json::json!({"token": token})),
//     }))
// }

// async fn login(
//     creds: web::Json<LoginRequest>,
//     state: web::Data<AppState>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let user = state.db
//         .find_one(doc! {"email": &creds.email}, None)
//         .await?
//         .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

//     if !verify(&creds.password, &user.password)? {
//         return Err(actix_web::error::ErrorUnauthorized("Invalid credentials"));
//     }

//     let token = generate_token(&user, &state.jwt_secret)?;

//     Ok(HttpResponse::Ok().json(SuccessResponse {
//         message: "Login successful".to_string(),
//         data: Some(serde_json::json!({"token": token})),
//     }))
// }

// fn generate_token(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
//     let claims = Claims {
//         email: user.email.clone(),
//         role: user.role.clone(),
//         exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
//         iat: Utc::now().timestamp() as usize,
//     };
    
//     encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();
    
//     let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your_secret_key".to_string());
//     let collection = setup_database().await.expect("Database connection failed");
    
//     let state = AppState {
//         db: collection,
//         jwt_secret,
//     };

//     HttpServer::new(move || {
//         App::new()
//             .wrap(middleware::Logger::default())
//             .app_data(web::Data::new(state.clone()))
//             .route("/api/signup", web::post().to(register))
//             .route("/api/login", web::post().to(login))
//     })
//     .bind(("127.0.0.1", 8081))?
//     .run()
//     .await
// }