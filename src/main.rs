use actix_web::middleware::Logger;
use actix_web::{get, post, App, HttpServer, Responder};
use std::env;
use std::error::Error;

#[get("/")]
async fn index() -> impl Responder {
    "Test server index"
}

#[post("/echo")]
async fn echo(data: String) -> impl Responder {
    data
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello from test server"
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .service(hello)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await?;

    Ok(())
}

fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "TRACE".into());
    let log_level = log_level.parse().unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = std::fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    builder.apply()?;

    log::trace!("TRACE output enabled");
    log::debug!("DEBUG output enabled");
    log::info!("INFO output enabled");
    log::warn!("WARN output enabled");
    log::error!("ERROR output enabled");

    Ok(())
}
