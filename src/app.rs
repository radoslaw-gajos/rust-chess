use warp::Filter;
use crate::routes;
use crate::store;

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn run(&self) {
        let db_url = "postgres://mydb:@localhost:3030";
        let store = store::Store::new(db_url).await;

        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Cannot migrate DB");

        let store_filter = warp::any().map(move || store.clone());

        let index = warp::path("static").and(warp::fs::dir("www/static"));
        let register = warp::post()
            .and(warp::path("register"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and_then(routes::authentication::register);

        let routes = index
            .or(register);

        warp::serve(routes).run(([127,0,0,1], 3030)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
