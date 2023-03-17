#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod daomanager {
    use ink::env::DefaultEnvironment;
    use ink::prelude::vec;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::Id;
    use openbrush::contracts::reentrancy_guard::*;
    use openbrush::traits::Storage;
    
    use toyota_pkg::{
        impls::daomanager::*,
        traits::daomanager::*,
    };


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DaoManagerContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        daomanager: types::Data,
    }

    impl DaoManagerContract {
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
                let mut instance = Self::default();
                instance.daomanager.token = token;
                let caller = instance.env().caller();
                instance._init_with_owner(caller.clone());
                instance
        }
        
    }

    impl DaoManager for DaoManagerContract {}

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::daomanager::DaoManagerContract;
        use ink::env::test;
        use openbrush::{
            contracts::psp34::Id,
            traits::String,
        };
        use toyota_pkg::impls::daomanager::types::DaoManagerError;

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let daomanager = init_contract();
            
            assert_eq!(daomanager.get_token(), token_address());
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn init_contract() -> DaoManagerContract {
            DaoManagerContract::new(token_address())
        }

        fn token_address() -> AccountId {
            AccountId::from([0x10; 32])
        }
    }
}