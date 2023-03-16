#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod dao {
    use ink::env::DefaultEnvironment;
    use ink::prelude::vec;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::Id;
    use openbrush::contracts::reentrancy_guard::*;
    use openbrush::traits::Storage;
    
    use toyota_pkg::{
        impls::dao::*,
        traits::dao::*,
    };


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DaoContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        dao: types::Data,
    }

    impl DaoContract {
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
            
                let mut instance = Self::default();
                instance.dao.token = token;
                instance.dao.quorum = 0; // 0%

                let caller = instance.env().caller();
                instance._init_with_owner(caller.clone());
                instance.dao.members = vec![caller];
                instance.dao.member_id = 1;
                instance
        }
        
    }

    impl ToyotaDao for DaoContract {}

    #[cfg(test)]
    mod Tests {
        use super::*;
        use crate::dao::DaoContract;
        use ink::env::test;
        use openbrush::{
            contracts::psp34::Id,
            traits::String,
        };
        use toyota_pkg::impls::dao::types::DaoError;

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let dao = init_contract();
            
            assert_eq!(dao.get_token_address(), token_address());
            assert_eq!(dao.get_number_of_members(),1u32);
        }

        #[ink::test]
        fn add_member_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert_eq!(dao.get_number_of_members(),1u32);

            assert!(dao.add_member(accounts.bob).is_ok());
            assert_eq!(dao.get_number_of_members(),2u32);
        }

        #[ink::test]
        fn create_project_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert!(dao.add_member(accounts.bob).is_ok());
            assert!(dao.add_member(accounts.charlie).is_ok());

            // Bob creates a new project
            set_sender(accounts.bob);
            assert_eq!(dao.get_number_of_projects(),0u32);
            assert!(dao.create_project(String::from("Project 1")).is_ok());
            assert_eq!(dao.get_number_of_projects(),1u32);

            // Charlie also creates a project
            set_sender(accounts.charlie);
            assert!(dao.create_project(String::from("Project 2")).is_ok());
            assert_eq!(dao.get_number_of_projects(),2u32);
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn init_contract() -> DaoContract {
            DaoContract::new(token_address())
        }

        fn token_address() -> AccountId {
            AccountId::from([0x10; 32])
        }
    }
}