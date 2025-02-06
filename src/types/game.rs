use uuid::Uuid;
use crate::types::account::AccountId;

#[derive(Debug, Clone)]
pub struct Game {
    pub uuid: Uuid,
    pub white: AccountId,
    pub black: AccountId,
}
