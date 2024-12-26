use warp::Filter;

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn run(&self) {
        let index = warp::path("static").and(warp::fs::dir("www/static"));

        warp::serve(index).run(([127,0,0,1], 3030)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
