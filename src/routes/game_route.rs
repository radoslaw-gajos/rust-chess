use warp::http::StatusCode;

use crate::store::Store;
use crate::types::account::Session;

pub async fn new_game(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.new_game(session.account_id).await
    {
        Ok(game_uuid) => Ok(warp::reply::json(&game_uuid.to_string())),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
