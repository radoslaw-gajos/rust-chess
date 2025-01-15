use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use crate::types::{
    account::{Account, AccountId},
};
use crate::handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await 
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection {}", e),
        };

        Self {
            connection: db_pool,
        }
    }

    pub async fn add_account(
        self,
        account: Account,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "insert into accounts (email, password) values ($1, $2)",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = err
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message =
                        err.as_database_error().unwrap().message(),
                    constraint = err
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(Error::DatabaseQueryError(err))
            },
        }
    }

    pub async fn get_account(
        self,
        email: String,
    ) -> Result<Account, Error> {
        match sqlx::query("select * from accounts where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }
}
