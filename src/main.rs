mod primitives;
mod requests_interfaces;
mod server;
mod auth;
mod token;
mod mongo;
mod redis;
mod users;
mod helpers;

use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env::var("HOST").expect("ADDRESS must be set");
    env::var("PORT").expect("PORT must be set");
    match env::var("PORT").unwrap().parse::<u16>() {
        Ok(_) => (),
        Err(_) => panic!("PORT must be a number"),
    };

    env::var("MONGO_URI").expect("DATABASE_URL must be set");
    env::var("MONGO_DB_NAME").expect("DB_NAME must be set");
    env::var("REDIS_URI").expect("REDIS_URI must be set");

    env::var("API_TOKENS_SIGNING_KEY").expect("API_SIGNING_KEY must be set");

    let expiration_time = match env::var("API_TOKENS_EXPIRATION_TIME") {
        Ok(expiration_time) => expiration_time,
        Err(_) => panic!("API_TOKENS_EXPIRATION_TIME not found"),
    };
    
    match expiration_time.parse::<usize>() {
        Ok(_) => (),
        Err(_) => panic!("API_TOKENS_EXPIRATION_TIME must be a number"),
    };

    let mongo_client = match mongo::init_connection().await {
        Ok(client) => client,
        Err(e) => panic!("Error connecting to MongoDB: {}", e),
    };

    let redis_connection = match redis::init_connection() {
        Ok(redis_connection) => redis_connection,
        Err(e) => panic!("Error connecting to Redis: {}", e),
    };

    match mongo_client.database("admin").run_command(mongodb::bson::doc! {"ping": 1}, None).await {
        Ok(_) => println!("Connected to MongoDB"),
        Err(e) => panic!("Error connecting to MongoDB: {}", e),
    };

    server::init(mongo_client, redis_connection).await;
}