use warp::{
    Rejection,
    Reply,
    http::StatusCode,
    reject::Reject,
};

#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
}

impl Reject for Error {
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        todo!();
    }
    Ok(warp::reply::with_status(
        "Internal Server Error".to_string(),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
