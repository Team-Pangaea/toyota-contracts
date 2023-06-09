use crate::{
    impls::daomanager::types::{
        Data,
        DaoId,
        DaoManagerError,
    },
    traits::daomanager::DaoManager,
    traits::dao::ToyotaDaoRef,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

pub trait Internal {

    fn is_member(&self,account: AccountId) -> bool;

    fn check_token(&self, token: AccountId) -> bool;

    fn check_eligible(&self,account: AccountId) -> bool;

    fn dao_exists(&self, dao: AccountId) -> bool;
}

impl<T> DaoManager for T
where
    T: Storage<Data> + Storage<ownable::Data> + Storage<reentrancy_guard::Data>,
{
    #[modifiers(only_owner)]
    default fn set_token(&mut self, token: AccountId) -> Result<(),DaoManagerError> {
        self.data::<Data>().token = token;
        Ok(())
    }

    default fn get_number_of_daos(&self) -> DaoId {
        self.data::<Data>().dao_id
    }

    default fn add_dao(&mut self, dao: AccountId) -> Result<(),DaoManagerError> {
        //0xbf76bd9d835d3f7d102df31935cb5a975c6fb1bf366fc55b60b152dcdac863c3
        let caller = Self::env().caller();

        let token = ToyotaDaoRef::get_token_address(&dao);

        if !self.is_member(caller.clone()) {
            return Err(DaoManagerError::NotAMember)
        }

        if !self.check_token(token.clone()) {
            return Err(DaoManagerError::WrongToken)
        }

        if self.dao_exists(dao.clone()) {
            return Err(DaoManagerError::DAOExists)
        }

        let dao_id = self.data::<Data>().dao_id.saturating_add(1);
        self.data::<Data>().dao_id = dao_id;

        self.data::<Data>().daos.push(dao.clone());

        Ok(())
    }

    default fn register(&mut self) -> Result<(),DaoManagerError> {
        let caller = Self::env().caller();

        if !self.check_eligible(caller.clone()) {
            return Err(DaoManagerError::NotEligible)
        }

        if self.is_member(caller.clone()) {
            return Err(DaoManagerError::AlreadyAMember)
        }

        let member_id = self.data::<Data>().member_id.saturating_add(1);
        self.data::<Data>().member_id = member_id;

        self.data::<Data>().members.push(caller.clone());


        Ok(())
    }

    default fn get_token(&self) -> AccountId {
        self.data::<Data>().token
    }

    default fn get_daos(&self) -> Vec<AccountId> {
        self.data::<Data>().daos.clone()
    }

    default fn get_members(&self) -> Vec<AccountId> {
        self.data::<Data>().members.clone()
    }

    default fn check_membership(&self,account: AccountId) -> bool {
        self.is_member(account.clone())
    }

}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn is_member(&self,account: AccountId) -> bool {

        if self.data::<Data>().members.contains(&account) {
            return true;
        } else {
            return false;
        }
        
    }

    default fn check_token(&self, token: AccountId) -> bool {
        if token == self.data::<Data>().token {
            return true;
        } else {
            return false;
        }
    }

    default fn check_eligible(&self,account: AccountId) -> bool {
        let address = self.data::<Data>().token;
        let balance = PSP34Ref::balance_of(&address, account.clone());

        if balance > 0 {
            return true;
        } else {
            return false;
        }
    }

    default fn dao_exists(&self, dao: AccountId) -> bool {
        if self.data::<Data>().daos.contains(&dao) {
            return true;
        } else {
            return false;
        }
    }

}