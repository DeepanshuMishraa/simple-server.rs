use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod db;
mod models;
mod schema;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}


#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[derive(Serialize)]
struct UserResponse {
    name: String,
    age: u32,
}

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().json(GreetResponse {
        message: "Hello, world".to_string(),
    })
}

#[get("/greet/{name}")]
async fn greet_name(name: web::Path<String>) -> impl Responder {
    let name = name.into_inner(); // get name from the Path 
    HttpResponse::Ok().json(GreetResponse {
        message: "Hello ".to_string() + &name,
    })
}

#[post("/user/create")]
async fn create_user(
    user_data: web::Json<User>,
    db_pool: web::Data<Arc<Mutex<PgConnection>>>,
) -> impl Responder {
    use crate::schema::users;

    let mut connection = db_pool.lock().unwrap();

    let new_user = models::NewUser {
        name: user_data.name.clone(),
        age: user_data.age as i32,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut *connection)
    {
        Ok(_) => HttpResponse::Created().json(UserResponse {
            name: user_data.name.clone(),
            age: user_data.age,
        }),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e)),
    }
}

#[get("/user/users")]
async fn get_users(db_pool: web::Data<Arc<Mutex<PgConnection>>>) -> impl Responder {
    use crate::schema::users;

    let mut connection = db_pool.lock().unwrap();

    let results = users::table
        .load::<models::DbUser>(&mut *connection)
        .expect("Error loading users");

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    println!("Server is running on port {}", port);

    let connection = db::connect_db(); // Get the actual connection
    let db_pool = Arc::new(Mutex::new(connection)); // Wrap it properly

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // Clone the Arc for each worker
            .service(greet)
            .service(greet_name)
            .service(create_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
