#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod dao {
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::reentrancy_guard::*;
    use openbrush::traits::Storage;
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        env::DefaultEnvironment,
        EnvAccess,
    };
    
    use toyota_pkg::{
        impls::dao::*,
        traits::dao::*,
    };
    use toyota_pkg::impls::dao::dao::DaoEvents;


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
        pub fn new(token: AccountId, metadata: Vec<u8>) -> Self {
            
                let mut instance = Self::default();
                instance.dao.token = token;
                instance.dao.quorum = 0; // 0%
                instance.dao.metadata = metadata;
                let caller = instance.env().caller();
                instance._init_with_owner(caller.clone());
                instance.dao.members = vec![caller];
                instance.dao.member_id = 1;
                instance
        }
        
    }

    impl DaoEvents for DaoContract {
        fn emit_member_added_event(&self, member:AccountId, member_id: u32) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<DaoContract>>::emit_event::<
            MemberAdded,
        >(
            self.env(),
            MemberAdded {
                member,
                member_id,
            },
        );
        }

        fn emit_project_created_event(&self, creator:AccountId, project_id: u32) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<DaoContract>>::emit_event::<
            ProjectCreated,
        >(
            self.env(),
            ProjectCreated {
                creator,
                project_id,
            },
        );
        }

        fn emit_proposal_created_event(&self, creator:AccountId, proposal_id: u32) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<DaoContract>>::emit_event::<
            ProposalCreated,
        >(
            self.env(),
            ProposalCreated {
                creator,
                proposal_id,
            },
        );
        }

        fn emit_task_created_event(&self, creator:AccountId, task_id: u32) {
            <EnvAccess<'_, DefaultEnvironment> as EmitEvent<DaoContract>>::emit_event::<
            TaskCreated,
        >(
            self.env(),
            TaskCreated {
                creator,
                task_id,
            },
        );
        }
    }

    impl ToyotaDao for DaoContract {}

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::DaoContract;
        use ink::env::test;
        use openbrush::{
            traits::String,
        };

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
            assert!(dao.create_task(accounts.bob,accounts.alice,duration,points,priority).is_ok());
            assert_eq!(dao.get_number_of_tasks(),1u32);

            assert_eq!(dao.get_member_points(accounts.bob),0u32);
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

            // Alice and Charlie join the project
            set_sender(accounts.alice);
            assert!(dao.join_project(1).is_ok());
            set_sender(accounts.charlie);
            assert!(dao.join_project(1).is_ok());

            let duration = 1000000;
            let points = 100;
            let priority = 1; //1,2 or 3

            //Bob creates a task
            set_sender(accounts.bob);
            assert_eq!(dao.get_number_of_project_tasks(1),0u32);
            assert!(dao.create_project_task(1,accounts.charlie,accounts.alice,duration,points,priority).is_ok());
            assert_eq!(dao.get_number_of_project_tasks(1),1u32);
            assert_eq!(dao.get_member_task_ids(accounts.charlie),vec![1]);

            // Bob creates a second task

            let duration2 = 10000;
            let points2 = 10;
            let priority2 = 2;

            assert!(dao.create_project_task(1,accounts.charlie,accounts.alice,duration2,points2,priority2).is_ok());
            assert_eq!(dao.get_number_of_project_tasks(1),2u32);
            assert_eq!(dao.get_member_task_ids(accounts.charlie),vec![1,2]);

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
            let metadata = String::from("Test");
            DaoContract::new(token_address(),metadata)
        }

        fn token_address() -> AccountId {
            AccountId::from([0x10; 32])
        }
    }
}