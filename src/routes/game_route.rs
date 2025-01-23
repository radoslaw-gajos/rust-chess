use warp::{http::StatusCode, Filter};

use crate::store::Store;
use crate::types::account::Account;

pub async fn new_game(
    store: Store,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.new_game(account).await
    {
        Ok(_) => Ok(warp::reply::with_status("Game created successfully", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
