use warp::Filter;
use dotenv::dotenv;
use std::env;

mod v1;
mod logger;

use logger::Logger;


// load .env file
fn load_env() -> Vec<String> {
    dotenv().ok();

    let env_var_value = match env::var("YOUR_ENV_VARIABLE") {
        Ok(value) => value,
        Err(_) => {
            Logger::panic("Environment variable not found");
            std::process::exit(1);
        }
    };

    env_var_value.split('.').map(String::from).collect()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // load .env file
    load_env();

    


    logger::Logger::info("Starting server on {}:{}", );
    logger::Logger::warn("This is a warning!");
    logger::Logger::error("This is an error!");

    v1::builtin_help();

    //print env variables
    println!("PORT: {}", env::var("API_PORT").unwrap());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let root = warp::path::end()
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));


    let routes = hello.or(root);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
