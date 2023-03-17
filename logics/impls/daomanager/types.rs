use ink::primitives::Hash;
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::OwnableError,
        psp34::Id,
        reentrancy_guard::ReentrancyGuardError,
    },
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        String,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use scale::{
    Decode,
    Encode,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
pub type DaoId = u32;

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub token: AccountId,
    pub daos: Vec<AccountId>,
    pub members: Vec<AccountId>,
    pub dao_id: DaoId,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            token: ZERO_ADDRESS.into(),
            daos: Default::default(),
            members: Default::default(),
            dao_id: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DaoManagerError {
     /// Caller is not the owner.
     OwnableError(OwnableError),
     /// Caller is tryin to make second call while 1st one is still executing.
     ReentrancyError(ReentrancyGuardError),
     /// Not Eligible,
     NotEligible,
     /// Already A Member
     AlreadyAMember,
     /// Not A Member
     NotAMember,
     /// Wrong Token
     WrongToken,
     /// DAO Exists
     DAOExists,
     
}

impl From<OwnableError> for DaoManagerError {
    fn from(error: OwnableError) -> Self {
        DaoManagerError::OwnableError(error)
    }
}

impl From<ReentrancyGuardError> for DaoManagerError {
    fn from(error: ReentrancyGuardError) -> Self {
        DaoManagerError::ReentrancyError(error)
    }
}

