use crate::store::Store;
use crate::types::account::Account;

fn async new_game(
    store: Store,
    account: Account,
) -> Result<impl warp::Reply, impl warp::Rejection> {
    todo!();
}
