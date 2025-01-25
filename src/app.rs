use warp::{http::Method, Filter};
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv;
use std::env;

use crate::routes;
use crate::store;
use crate::handle_errors;

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn run(&self) {
        dotenv::dotenv().ok();

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO key not set");
        }

        let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
            "chess=warn,warp=warn".to_owned()
        });

        let db_url = "postgres://localhost:5432/mydb";
        let store = store::Store::new(db_url).await;

        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Cannot migrate DB");

        let store_filter = warp::any().map(move || store.clone());

        tracing_subscriber::fmt()
            .with_env_filter(log_filter)
            .with_span_events(FmtSpan::CLOSE)
            .init();

        let cors = warp::cors()
            .allow_any_origin() // todo: change for production
            .allow_header("content-type")
            .allow_methods(&[
                Method::PUT,
                Method::DELETE,
                Method::POST,
                Method::GET,
            ]);

        let index = warp::path("static").and(warp::fs::dir("www/static"));
        let register = warp::post()
            .and(warp::path("register"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::body::json())
            .and_then(routes::authentication::register);

        let login = warp::post()
            .and(warp::path("login"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::body::json())
            .and_then(routes::authentication::login);

        let new_game = warp::post()
            .and(warp::path("game"))
            .and(warp::path::end())
            .and(routes::authentication::auth())
            .and(store_filter.clone())
            .and_then(routes::game_route::new_game);


        let routes = index
            .or(register)
            .or(login)
            .or(new_game)
            .with(cors)
            .with(warp::trace::request())
            .recover(handle_errors::return_error);

        warp::serve(routes).run(([127,0,0,1], 3030)).await;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
