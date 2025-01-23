use uuid::Uuid;
use crate::types::account::AccountId;

#[derive(Debug, Clone)]
pub struct Game {
    uuid: Uuid,
    white: AccountId,
    black: AccountId,
}
