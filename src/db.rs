use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub fn connect_db()->PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");
    
    match database_url {
        Ok(url)=>{
            PgConnection::establish(&url);
            println!("Connected to database");
        },
        Err(_) => {
            panic!("DATABASE_URL not set in .env file");
        }
    }
}
