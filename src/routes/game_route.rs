use warp::http::StatusCode;

use crate::store::Store;
use crate::types::account::Session;

pub async fn new_game(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.new_game(session.account_id).await
    {
        Ok(_) => Ok(warp::reply::with_status("Game created successfully", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
