use crate::impls::dao::types::{
    DaoError,
    Proposal,
    Vote,
};
use ink::primitives::Hash;
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
        BlockNumber,
    },
};

#[openbrush::trait_definition]
pub trait ToyotaDao {

    #[ink(message)]
    fn add_member(&mut self,address: AccountId) -> Result<(),DaoError>;

    #[ink(message)]
    fn join_dao(&mut self) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_proposal(&mut self,description: String, duration: Timestamp) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_task(&mut self,assignee: AccountId, reviewer: AccountId, duration: Timestamp, points: u32, priority: u8) -> Result<(),DaoError>;

    #[ink(message)]
    fn get_token_address(&self) -> AccountId;

    #[ink(message)]
    fn get_quorum(&self) -> u8;
}