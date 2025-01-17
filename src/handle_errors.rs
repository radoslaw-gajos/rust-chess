use argon2::Error as ArgonError;
use warp::{
    Rejection,
    Reply,
    http::StatusCode,
    reject::Reject,
};
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
    WrongPassword,
    ArgonLibraryError(ArgonError),
    CannotDecryptToken,
}

impl Reject for Error {
}

const DUPLICATE_KEY: u32 = 23505;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DatabaseQueryError(_) => {
                write!(f, "Cannot update, invalid data")
            },
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::ArgonLibraryError(_) => {
                write!(f, "Cannot verify password")
            },
            Error::CannotDecryptToken => write!(f, "Cannot decrypt token error"),
        }
    }
}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap()
                    == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Entity already exists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            },
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        event!(Level::ERROR, "Internal server error");
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
