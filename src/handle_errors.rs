use warp::{
    Rejection,
    Reply,
    http::StatusCode,
};

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(
        "Internal Server Error".to_string(),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
