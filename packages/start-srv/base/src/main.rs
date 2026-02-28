//! Universo Platformo | Start Server
//!
//! Actix Web backend for the start page functionality.
//! Provides authentication (via Supabase) and onboarding API endpoints.

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;

mod config;
mod error;
mod handlers;
mod models;
mod supabase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::AppConfig::from_env();
    let host = config.host.clone();
    let port = config.port;
    let secret_key = Key::from(config.session_secret.as_bytes());
    let cookie_secure = config.cookie_secure;
    let cors_allowed_origins = config.cors_allowed_origins.clone();
    let supabase_client = web::Data::new(supabase::SupabaseClient::new(
        config.supabase_url.clone(),
        config.supabase_anon_key.clone(),
    ));

    log::info!("Starting start-srv on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = if cors_allowed_origins.is_empty() {
            // Development: allow all origins (shows warning in config loading)
            Cors::default()
                .allowed_origin_fn(|_, _| true)
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600)
        } else {
            // Production: restrict to configured origins
            let mut builder = Cors::default()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600);
            for origin in &cors_allowed_origins {
                builder = builder.allowed_origin(origin);
            }
            builder
        };

        let session_middleware =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(cookie_secure)
                .build();

        App::new()
            .app_data(supabase_client.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(session_middleware)
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("/logout", web::post().to(handlers::auth::logout))
                            .route("/me", web::get().to(handlers::auth::me)),
                    )
                    .service(
                        web::scope("/onboarding")
                            .route("/items", web::get().to(handlers::onboarding::get_items))
                            .route("/join", web::post().to(handlers::onboarding::join_items)),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
