use reqwest::StatusCode;

pub async fn register() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Todo!", StatusCode::OK))
}
