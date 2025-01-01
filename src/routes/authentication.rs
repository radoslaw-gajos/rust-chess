use reqwest::StatusCode;
use crate::store::Store;

pub async fn register(
    store: Store,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    //let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Account added", StatusCode::OK))
        }
        Err(e) => todo!(),
    }
}
