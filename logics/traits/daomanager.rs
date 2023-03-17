use crate::impls::daomanager::types::{
    DaoManagerError,
    DaoId,
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

#[openbrush::wrapper]
pub type DaoManagerRef = dyn DaoManager;

#[openbrush::trait_definition]
pub trait DaoManager {
    #[ink(message)]
    fn get_number_of_daos(&self) -> DaoId;

    #[ink(message)]
    fn set_token(&mut self, token: AccountId) -> Result<(),DaoManagerError>;

    #[ink(message)]
    fn add_dao(&mut self, dao: AccountId) -> Result<(),DaoManagerError>;

    #[ink(message)]
    fn register(&mut self, account: AccountId) -> Result<(),DaoManagerError>;

    #[ink(message)]
    fn get_token(&self) -> AccountId;

    #[ink(message)]
    fn get_daos(&self) -> Vec<AccountId>;

    #[ink(message)]
    fn get_members(&self) -> Vec<AccountId>;

}