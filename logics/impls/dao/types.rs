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
pub type ProposalId = u32;
pub type TokenId = u32;
pub type ProjectId = u32;
pub type TaskId = u32;

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub proposal: Mapping<ProposalId,Proposal>,
    pub vote: Mapping<ProposalId,Vote>,
    pub task: Mapping<TaskId,Task>,
    pub project: Mapping<ProjectId,Project>,
    pub members: Vec<AccountId>,
    pub member_token: Mapping<AccountId,TokenId>,
    pub member_points: Mapping<AccountId,u32>,
    pub member_votes: Mapping<(AccountId,ProposalId),bool>,
    pub project_tasks: Mapping<ProjectId,Vec<TaskId>>,
    pub project_members: Mapping<ProjectId,Vec<AccountId>>,
    pub token: AccountId,
    pub quorum: u32,
    pub proposal_id: u32,
    pub member_id: u32,
    pub project_id: u32,
    pub task_id: u32,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            proposal: Default::default(),
            vote: Default::default(),
            task: Default::default(),
            project: Default::default(),
            members: Default::default(),
            member_token: Default::default(),
            member_points: Default::default(),
            member_votes: Default::default(),
            project_tasks: Default::default(),
            project_members: Default::default(),
            token: ZERO_ADDRESS.into(),
            quorum: 0,
            proposal_id: 0,
            member_id: 0,
            project_id: 0,
            task_id: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DaoError {
     /// Caller is not the owner.
     OwnableError(OwnableError),
     /// Caller is tryin to make second call while 1st one is still executing.
     ReentrancyError(ReentrancyGuardError),
     /// Member Already Exists
     MemberAlreadyExists,
     /// Member does not exist
     MemberDoesNotExist,
     /// Not Eligible for Membership
     NotEligibleForMembership,
     /// Wrong Task priority
     WrongTaskPriority,
     /// Member Exists in Project
     MemberExistsInProject,
     /// Task Does Not Exist
     TaskDoesNotExist,
     /// Ineligible Caller
     IneligibleCaller,
     /// Member has already Voted
     MemberHasAlreadyVoted,
     /// Voting Period Expired
     VotingPeriodExpired,
     /// Proposal Does Not Exist
     ProposalDoesNotExist,
     /// Vote Not Available
     VoteNotAvailable,
     /// Vote Ongoing
     VoteOngoing,
     /// Quorum Not Achieved
     QuorumNotAchieved,

}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Project {
    pub creator: AccountId,
    pub description: String,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            creator: ZERO_ADDRESS.into(),
            description: Default::default(),
        }
    }
}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Proposal {
    pub creator: AccountId,
    pub description: String,
}

impl Default for Proposal {
    fn default() -> Self {
        Self {
            creator: ZERO_ADDRESS.into(),
            description: Default::default(),
        }
    }
}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]

pub struct Vote {
    pub yes_votes: u32,
    pub no_votes: u32,
    pub start: Timestamp,
    pub end: Timestamp,
    pub vote_status: VoteStatus,
}

impl Default for Vote {
    fn default() -> Self {
        Self {
            yes_votes: 0,
            no_votes: 0,
            start: 0,
            end: 0,
            vote_status: VoteStatus::NotAvailable,
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum VoteStatus {
    NotAvailable,
    InProgress,
	Passed,
	Failed,
}

#[derive(Encode, Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Task {
    pub assignee: AccountId,
    pub reviewer: AccountId,
    pub owner: AccountId,
    pub deadline: Timestamp,
    pub points: u32,
    pub priority: TaskPriority,
    pub status: TaskStatus,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            assignee: ZERO_ADDRESS.into(),
            reviewer: ZERO_ADDRESS.into(),
            owner: ZERO_ADDRESS.into(),
            deadline: 0,
            points: 0,
            priority: TaskPriority::None,
            status: TaskStatus::ToDo,
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum TaskPriority {
    None,
    Low,
    Moderate,
    High,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub enum TaskStatus {
    ToDo,
    InProgress,
    UnderReview,
    Done,
}

impl From<OwnableError> for DaoError {
    fn from(error: OwnableError) -> Self {
        DaoError::OwnableError(error)
    }
}

impl From<ReentrancyGuardError> for DaoError {
    fn from(error: ReentrancyGuardError) -> Self {
        DaoError::ReentrancyError(error)
    }
}