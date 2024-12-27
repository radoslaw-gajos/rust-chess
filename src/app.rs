use warp::Filter;
use crate::routes;

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn run(&self) {
        let index = warp::path("static").and(warp::fs::dir("www/static"));
        let register = warp::post()
            .and(warp::path("register"))
            .and(warp::path::end())
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
