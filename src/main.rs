use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct User{
    name:String,
    age:u32,
}

#[derive(Serialize)]
struct GreetResponse{
    message:String,
}

type Db = Arc<Mutex<HashMap<u32,User>>>; // Mutex is to implement mutual exclusion so that no two
                                         // threads can access the data at the same time.
#[derive(Serialize)]
struct UserResponse{
    id:u32,
    name:String,
    age:u32,
}


#[get("/greet")]
async fn greet()-> impl Responder{
    HttpResponse::Ok().json(GreetResponse{
        message:"Hello, world".to_string(),
    })
}

#[get("/greet/{name}")]
async fn greet_name(name: web::Path<String>)->impl Responder{
    let name = name.into_inner(); // get name from the Path 
    HttpResponse::Ok().json(GreetResponse{
        message:"Hello ".to_string() + &name,
    })
}

#[post("/user/create")]
async fn create_user(user_data:web::Json<User>,db:web::Data<Db>)->impl Responder{
    let mut db = db.lock().unwrap(); //lock the db 

    let new_id = db.keys().max().unwrap_or(&0) + 1; // get the max id and increment it by 1
                                                    
    let name = user_data.name.clone(); // clone the name to avoid ownership issues
    let age = user_data.age; // get the age

    db.insert(new_id,user_data.into_inner()); // insert the new user into the db

    HttpResponse::Created().json(UserResponse{
        id: new_id,
        name,
        age,
    })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000; 
    println!("Server is running on port {}",port);

    let user_db:Db = Arc::new(Mutex::new(HashMap::<u32, User>::new())); // create a new database

    HttpServer::new(move ||
        {
            let app_data = web::Data::new(user_db.clone());
            App::new().app_data(app_data).service(greet).service(greet_name).service(create_user)
        }).
    bind(("127.0.0.1",port))?
    .workers(2)
    .run()
    .await
}
