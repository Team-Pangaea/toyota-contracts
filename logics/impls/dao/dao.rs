use crate::{
    impls::dao::types::{
        Data,
        Proposal,
        DaoError,
        Vote,
        VoteStatus,
        Task,
        TaskStatus,
        TaskPriority,
    },
    traits::dao::ToyotaDao,
};
use ink::primitives::Hash;
use ink::prelude::vec::Vec;
//use ink::Blake2x256;
   
use openbrush::{
    contracts::{
        ownable::*,
        psp34::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
        Timestamp,
        BlockNumber,
    },
};
use ink::ToAccountId;

pub trait Internal {
    fn is_eligible(&self,account: AccountId) -> bool;
}

impl<T> ToyotaDao for T
where
    T: Storage<Data> + Storage<ownable::Data> + Storage<reentrancy_guard::Data>,
{

    #[modifiers(only_owner)]
    default fn add_member(&mut self,address: AccountId) -> Result<(),DaoError> {
        if self.data::<Data>().members.contains(&address) {
            return Err(DaoError::MemberAlreadyExists)
        }

        // Dummy

        let member_id = self.data::<Data>().member_id.saturating_add(1);
        self.data::<Data>().member_id = member_id;

        // Add to the members

        self.data::<Data>().members.push(address.clone());
        self.data::<Data>().member_token.insert(&address,&member_id);

        Ok(())
    }

    default fn join_dao(&mut self) -> Result<(),DaoError> {
        let address = Self::env().caller();
        if self.data::<Data>().members.contains(&address) {
            return Err(DaoError::MemberAlreadyExists)
        }

        if !self.is_eligible(address.clone()) {
            return Err(DaoError::NotEligibleForMembership)
        }

        let member_id = self.data::<Data>().member_id.saturating_add(1);
        self.data::<Data>().member_id = member_id;

        self.data::<Data>().members.push(address.clone());
        self.data::<Data>().member_token.insert(&address,&member_id);

        Ok(())

    }

    default fn create_proposal(&mut self,description: String, duration: Timestamp) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        let now = Self::env().block_timestamp();

        let vote = Vote {
            yes_votes: 0,
            no_votes: 0,
            start: now,
            end: now + duration,
            vote_status: VoteStatus::InProgress,
        };

        let proposal_id = self.data::<Data>().proposal_id.saturating_add(1);

        self.data::<Data>().proposal.insert(&proposal_id.clone(),
            &Proposal {
                creator: caller.clone(),
                description: description,
                vote: vote,
            });


        self.data::<Data>().proposal_id = proposal_id;

        Ok(())
    }

    default fn create_task(&mut self,assignee: AccountId, reviewer: AccountId, duration: Timestamp, points: u32, priority: u8) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        let now = Self::env().block_timestamp();

        let task_priority = match priority {
            1 => TaskPriority::Low,
            2 => TaskPriority::Moderate,
            3 => TaskPriority::High,
            _ => return Err(DaoError::WrongTaskPriority)
        };

        let task = Task {
            assignee: assignee,
            reviewer: reviewer,
            owner: caller.clone(),
            deadline: now + duration,
            points: points,
            priority: task_priority,
            status: TaskStatus::ToDo,
        };

        let task_id = self.data::<Data>().task_id.saturating_add(1);

        self.data::<Data>().task.insert(&task_id.clone(),&task);


        self.data::<Data>().task_id = task_id;

        Ok(())
    }

    default fn get_token_address(&self) -> AccountId {
        self.data::<Data>().token
    }

    default fn get_quorum(&self) -> u8 {
        self.data::<Data>().quorum
    }

}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn is_eligible(&self,_account: AccountId) -> bool {
        return true;
    }

}