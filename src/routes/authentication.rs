use reqwest::StatusCode;
use crate::store::Store;
use crate::types::account::Account;
use argon2::Config;
use rand::Rng;

pub async fn register(
    store: Store,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account = Account {
        password: hash_password(account.password.as_bytes()),
        ..account
    };

    match store.add_account(account).await
    {
        Ok(bool) => Ok(warp::reply::with_status("Todo", StatusCode::OK)),
        Err(_) => todo!(),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
