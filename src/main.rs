use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to load configuration");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("localhost:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
