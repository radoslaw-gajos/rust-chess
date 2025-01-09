use argon2::Config;
use rand::Rng;
use chrono::prelude::*;
use warp::{http::StatusCode, Filter};
use std::future;

use crate::store::Store;
use crate::types::account::{Account, AccountId, Session};

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
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(
            //todo: make a list not hardcoded
            "conversation terminal deficit wi".as_bytes(),
        ))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token with builder!")
}

fn verify_token(
    token: String,
) -> Result<Session, ()> {
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        //todo: list of words not hardcoded
        &"conversation terminal deficit wi".as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| todo!())?;

    serde_json::from_value::<Session>(token)
        .map_err(|_| todo!())
}

pub fn auth(
) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };

        future::ready(Ok(token))
    })
}
