use std::io::Error;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port");
    run(listener)?.await
}