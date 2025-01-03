use reqwest::StatusCode;
use crate::store::Store;
use crate::types::account::Account;

pub async fn register(
    store: Store,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Todo", StatusCode::OK))
}
