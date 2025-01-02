use reqwest::StatusCode;
use crate::store::Store;

pub async fn register(
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Todo", StatusCode::OK))
}
