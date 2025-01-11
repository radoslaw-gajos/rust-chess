use argon2::Error as ArgonError;
use warp::{
    Rejection,
    Reply,
    http::StatusCode,
    reject::Reject,
};

#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
    WrongPassword,
    ArgonLibraryError(ArgonError),
}

impl Reject for Error {
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        todo!();
    } else if let Some(Error::WrongPassword) = r.find() {
        Ok(warp::reply::with_status(
            "Wrong E-mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
