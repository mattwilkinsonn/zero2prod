use std::net::TcpListener;

use zero2prod::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:4000").expect("Failed to bind listener");
    run(listener)?.await
}
