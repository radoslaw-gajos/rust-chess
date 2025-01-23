use uuid::Uuid;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use crate::types::{
    account::{Account, AccountId},
    game::{Game},
};
use crate::handle_errors::Error;
use tracing::{Level, event};

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
                event!(
                    Level::ERROR,
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
                event!(Level::ERROR, "{:?}", err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }

    pub async fn new_game(
        self,
        account: Account,
    ) -> Result<Uuid, Error> {
        match sqlx::query("SELECT uuid FROM games WHERE black IS NULL OR white IS NULL")
            .map(|row: PgRow| Uuid::parse_str(row.get("uuid")))
            .fetch_one(&self.connection)
            .await
        {
            Ok(uuid) => Ok(uuid.unwrap()),
            Err(err) => {
                self.create_game(account).await
            },
        }
    }

    pub async fn create_game(
        self,
        account: Account,
    ) -> Result<Uuid, Error> {
        let uuid = Uuid::new_v4();

        match sqlx::query("INSERT INTO games (uuid, white) VALUES ($1, $2) RETURNING uuid")
            .bind(uuid.to_string())
            .bind(account.id.unwrap().0)
            .map(|row: PgRow| Uuid::parse_str(row.get("uuid")))
            .fetch_one(&self.connection)
            .await
        {
            Ok(uuid) => Ok(uuid.unwrap()),
            Err(err) => {
                event!(Level::ERROR, "{:?}", err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }
}
