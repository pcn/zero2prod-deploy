// use std::net::TcpListener;
// use zero2prod::startup::{run};

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");

//     run(listener)?.await
// }

use zero2prod::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to localhost port 8000");
    run(listener)?.await
}
