use reqwest::StatusCode;
use crate::store::Store;
use crate::types::account::{Account, AccountId};
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

pub async fn login(
    store: Store,
    login: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => {
            match verify_password(
                &account.password,
                login.password.as_bytes(),
            ) {
                Ok(verified) => {
                    if verified {
                        Ok(warp::reply::json(&issue_token(
                            account.id.expect("id not found"),
                        )))
                    } else {
                        todo!();
                    }
                },
                Err(_) => todo!(),
            }
        },
        Err(_) => todo!(),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(
    hash: &str,
    password: &[u8],
) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    "todo".to_string()
}
