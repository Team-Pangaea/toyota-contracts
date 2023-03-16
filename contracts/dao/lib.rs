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

    #[ink(event)]
    pub struct MemberAdded {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        member_id: u32,
    }

    #[ink(event)]
    pub struct ProjectCreated {
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        project_id: u32,
    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        proposal_id: u32,
    }

    #[ink(event)]
    pub struct TaskCreated {
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        task_id: u32,
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
    mod tests {
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

            assert_eq!(dao.get_members(),vec![accounts.alice,accounts.bob]);
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

        #[ink::test]
        fn join_project_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert!(dao.add_member(accounts.bob).is_ok());
            assert!(dao.add_member(accounts.charlie).is_ok());

            // Bob creates a new project
            set_sender(accounts.bob);
            assert!(dao.create_project(String::from("Project 1")).is_ok());

            // Bob joins the project
            assert!(dao.join_project(1).is_ok());
            assert_eq!(dao.get_project_members(1),vec![accounts.bob]);
        }

        #[ink::test]
        fn create_task_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert!(dao.add_member(accounts.bob).is_ok());

            let duration = 1000000;
            let points = 100;
            let priority = 1; //1,2 or 3

            assert_eq!(dao.get_number_of_tasks(),0u32);
            assert!(dao.create_task(accounts.alice,accounts.alice,duration,points,priority).is_ok());
            assert_eq!(dao.get_number_of_tasks(),1u32);
        }

        #[ink::test]
        fn create_project_task_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert!(dao.add_member(accounts.bob).is_ok());
            assert!(dao.add_member(accounts.charlie).is_ok());

            // Bob creates a new project
            set_sender(accounts.bob);
            assert!(dao.create_project(String::from("Project 1")).is_ok());

            // Bob joins the project
            assert!(dao.join_project(1).is_ok());

            let duration = 1000000;
            let points = 100;
            let priority = 1; //1,2 or 3

            //Bob creates a task
            assert_eq!(dao.get_number_of_project_tasks(1),0u32);
            assert!(dao.create_project_task(1,accounts.alice,accounts.alice,duration,points,priority).is_ok());
            assert_eq!(dao.get_number_of_project_tasks(1),1u32);

            // Bob creates a second task

            let duration2 = 10000;
            let points2 = 10;
            let priority2 = 2;

            assert!(dao.create_project_task(1,accounts.alice,accounts.alice,duration2,points2,priority2).is_ok());
            assert_eq!(dao.get_number_of_project_tasks(1),2u32);

        }

        #[ink::test]
        fn create_proposal_works() {
            let accounts = default_accounts();
            set_sender(accounts.alice);

            let mut dao = init_contract();
            assert!(dao.add_member(accounts.bob).is_ok());
            assert!(dao.add_member(accounts.charlie).is_ok());

            let duration = 100000;

            // Bob creates a new proposal
            set_sender(accounts.bob);
            assert_eq!(dao.get_number_of_proposals(),0u32);
            assert!(dao.create_proposal(String::from("Proposal 1"),duration).is_ok());
            assert_eq!(dao.get_number_of_proposals(),1u32);

            // Check Vote count
            assert_eq!(dao.get_current_vote_count(1),(0u32,0u32));

            // Charlie casts a vote
            set_sender(accounts.charlie);
            assert!(dao.vote(1,true).is_ok());
            assert_eq!(dao.get_current_vote_count(1),(1u32,0u32));

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