use crate::impls::dao::types::{
    DaoError,
    Project,
    Proposal,
    Vote,
    ProjectId,
    ProposalId,
    TaskId,
    Task,
};
use ink::prelude::vec::Vec;
use openbrush::{
    traits::{
        AccountId,
        String,
        Timestamp,
    },
};

#[openbrush::wrapper]
pub type ToyotaDaoRef = dyn ToyotaDao;

#[openbrush::trait_definition]
pub trait ToyotaDao {

    #[ink(message)]
    fn add_member(&mut self,address: AccountId) -> Result<(),DaoError>;

    #[ink(message)]
    fn join_dao(&mut self) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_proposal(&mut self,description: String, duration: Timestamp) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_project(&mut self, description: String) -> Result<(),DaoError>;

    #[ink(message)]
    fn vote(&mut self, proposal_id: ProposalId, vote_cast: bool) -> Result<(),DaoError>;

    #[ink(message)]
    fn finalize_vote(&mut self, proposal_id: ProposalId) -> Result<(),DaoError>;

    #[ink(message)]
    fn join_project(&mut self, project_id: ProjectId ) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_task(&mut self,assignee: AccountId, reviewer: AccountId, duration: Timestamp, points: u32, priority: u8) -> Result<(),DaoError>;

    #[ink(message)]
    fn create_project_task(&mut self, project_id: ProjectId, assignee: AccountId, reviewer: AccountId, duration: Timestamp,
        points: u32, priority: u8) -> Result<(),DaoError>;

    #[ink(message)]
    fn start_task(&mut self, task_id: TaskId) -> Result<(),DaoError>;

    #[ink(message)]
    fn submit_task(&mut self, task_id: TaskId) -> Result<(),DaoError>;

    #[ink(message)]
    fn review_task(&mut self, task_id: TaskId, review: String, awarded_points: u32) -> Result<(),DaoError>;

    #[ink(message)]
    fn get_token_address(&self) -> AccountId;

    #[ink(message)]
    fn get_quorum(&self) -> u32;

    #[ink(message)]
    fn get_members(&self) -> Vec<AccountId>;

    #[ink(message)]
    fn get_project_members(&self,project_id: ProjectId) -> Vec<AccountId>;

    #[ink(message)]
    fn get_number_of_members(&self) -> u32;

    #[ink(message)]
    fn get_number_of_projects(&self) -> u32;

    #[ink(message)]
    fn get_number_of_tasks(&self) -> u32;

    #[ink(message)]
    fn get_task(&self, task_id: TaskId) -> Task;

    #[ink(message)]
    fn get_project_task_ids(&self,project_id: ProjectId) -> Vec<TaskId>;

    #[ink(message)]
    fn get_member_points(&self, assignee: AccountId) -> u32;

    #[ink(message)]
    fn get_member_task_ids(&self, assignee:AccountId) -> Vec<TaskId>;

    #[ink(message)]
    fn get_member_proposal_ids(&self, assignee:AccountId) -> Vec<ProposalId>;

    #[ink(message)]
    fn get_project(&self,project_id: ProjectId) -> Project;

    #[ink(message)]
    fn get_proposal(&self,proposal_id: ProposalId) -> Proposal;

    #[ink(message)]
    fn get_number_of_project_tasks(&self,project_id: ProjectId) -> u32;

    #[ink(message)]
    fn get_proposal_vote(&self,proposal_id: ProposalId) -> Vote;

    #[ink(message)]
    fn get_current_vote_count(&self,proposal_id: ProposalId) -> (u32,u32);

    #[ink(message)]
    fn get_number_of_proposals(&self) -> u32;
}