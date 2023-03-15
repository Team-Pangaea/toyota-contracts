#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod dao {
    use ink::env::DefaultEnvironment;
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
                instance._init_with_owner(caller);
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
            let dao = init_contract();
            assert_eq!(dao.get_token_address(), token_address());
        }

        fn init_contract() -> DaoContract {
            DaoContract::new(token_address())
        }

        fn token_address() -> AccountId {
            AccountId::from([0x1; 32])
        }
    }
}