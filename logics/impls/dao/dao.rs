use crate::{
    impls::dao::types::{
        Data,
        Proposal,
        ProposalId,
        Project,
        DaoError,
        Vote,
        VoteStatus,
        Task,
        TaskStatus,
        TaskId,
        TaskPriority,
        ProjectId,
    },
    traits::dao::ToyotaDao,
};
use ink::primitives::Hash;
use ink::prelude::vec::Vec;
use ink::prelude::vec;
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

    fn is_project_member(&self,project_id: ProjectId,account: AccountId) -> bool;

    fn create_task_internal(&mut self,caller: AccountId,assignee: AccountId, reviewer: AccountId, deadline: Timestamp,
         task_priority: TaskPriority, points: u32) -> TaskId;
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
    
    default fn vote(&mut self, proposal_id: ProposalId, vote: bool) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }


        Ok(())
    }

    default fn create_project(&mut self, description: String) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        let project_id = self.data::<Data>().project_id.saturating_add(1);

        self.data::<Data>().project.insert(&project_id.clone(),
            &Project {
                creator: caller.clone(),
                description: description,
            });


        self.data::<Data>().project_id = project_id;

        Ok(())
    }

    default fn join_project(&mut self, project_id: ProjectId ) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        let project_members = self.data::<Data>().project_members.get(&project_id);

        if let Some(mut members) = project_members {
            if members.contains(&caller) {
                return Err(DaoError::MemberExistsInProject)
            }
            members.push(caller.clone());
            self.data::<Data>().project_members.insert(&project_id, &members);
        } else {
            let members = vec![caller.clone()];
            self.data::<Data>().project_members.insert(&project_id, &members);
        }

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

    default fn create_project_task(&mut self, project_id: ProjectId, assignee: AccountId, reviewer: AccountId, duration: Timestamp,
    points: u32, priority: u8) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        let now = Self::env().block_timestamp();

        let deadline = now + duration;

        let task_priority = match priority {
            1 => TaskPriority::Low,
            2 => TaskPriority::Moderate,
            3 => TaskPriority::High,
            _ => return Err(DaoError::WrongTaskPriority)
        };

        let task_id = self.create_task_internal(caller.clone(),assignee.clone(),reviewer.clone(),deadline,task_priority,points);

        let project_tasks = self.data::<Data>().project_tasks.get(&project_id);

        if let Some(mut tasks) = project_tasks {
            tasks.push(task_id.clone());
            self.data::<Data>().project_tasks.insert(&project_id, &tasks);
        } else {
            let tasks = vec![task_id.clone()];
            self.data::<Data>().project_tasks.insert(&project_id, &tasks);
        }


        Ok(())
    }

    default fn start_task(&mut self, task_id: TaskId) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if self.data::<Data>().task.get(&task_id).is_none() {
            return Err(DaoError::TaskDoesNotExist);
        }

        let mut task = self.data::<Data>().task.get(&task_id).unwrap();

        if task.assignee != caller {
            return Err(DaoError::IneligibleCaller)
        }

        task.status = TaskStatus::InProgress;

        self.data::<Data>().task.insert(&task_id,&task);

        Ok(())
    }

    default fn submit_task(&mut self, task_id: TaskId) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if self.data::<Data>().task.get(&task_id).is_none() {
            return Err(DaoError::TaskDoesNotExist);
        }

        let mut task = self.data::<Data>().task.get(&task_id).unwrap();

        if task.assignee != caller {
            return Err(DaoError::IneligibleCaller)
        }

        task.status = TaskStatus::UnderReview;

        self.data::<Data>().task.insert(&task_id,&task);

        Ok(())
    }

    default fn review_task(&mut self, task_id: TaskId, awarded_points: u32) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if self.data::<Data>().task.get(&task_id).is_none() {
            return Err(DaoError::TaskDoesNotExist);
        }

        let mut task = self.data::<Data>().task.get(&task_id).unwrap();

        if task.reviewer != caller {
            return Err(DaoError::IneligibleCaller)
        }

        task.status = TaskStatus::Done;

        let assignee = task.assignee;

        let member_points = self.data::<Data>().member_points.get(&assignee);

        if let Some(mut points) = member_points {
            points = points + awarded_points;
            self.data::<Data>().member_points.insert(&assignee, &points);
        } else {
            let points = awarded_points;
            self.data::<Data>().member_points.insert(&assignee, &points);
        }

        self.data::<Data>().task.insert(&task_id,&task);

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

    default fn is_project_member(&self,project_id: ProjectId,account: AccountId) -> bool {

        let project_members = self.data::<Data>().project_members.get(&project_id);

        if let Some(mut members) = project_members {
            if members.contains(&account) {
                return true;
            } else {
                return false;
            }
            
        } else {
            return false;
        }
    }

    default fn create_task_internal(&mut self,caller: AccountId,assignee: AccountId, reviewer: AccountId, deadline: Timestamp,
         task_priority: TaskPriority, points: u32) -> TaskId {
        
        let task = Task {
            assignee: assignee,
            reviewer: reviewer,
            owner: caller.clone(),
            deadline: deadline,
            points: points,
            priority: task_priority,
            status: TaskStatus::ToDo,
        };

        let task_id = self.data::<Data>().task_id.saturating_add(1);

        self.data::<Data>().task.insert(&task_id.clone(),&task);


        self.data::<Data>().task_id = task_id;

        task_id
    }

}