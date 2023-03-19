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
        Storage,
        String,
        Timestamp,
        ZERO_ADDRESS,
    },
};

pub trait Internal {
    fn is_eligible(&self,account: AccountId) -> bool;

    fn is_project_member(&self,project_id: ProjectId,account: AccountId) -> bool;

    fn create_task_internal(&mut self,description: String, caller: AccountId,assignee: AccountId, reviewer: AccountId, deadline: Timestamp,
         task_priority: TaskPriority, points: u32) -> TaskId;
}

pub trait DaoEvents {
    fn emit_member_added_event(&self, member:AccountId, member_id: u32);
    fn emit_project_created_event(&self, creator:AccountId, project_id: u32);
    fn emit_proposal_created_event(&self, creator:AccountId, proposal_id: u32);
    fn emit_task_created_event(&self, creator:AccountId, task_id: u32);
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

        self.emit_member_added_event(address,member_id);

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

        self.emit_member_added_event(address,member_id);

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
        });

        let member_proposals = self.data::<Data>().member_proposals.get(&caller);

        if let Some(mut proposals) = member_proposals {
            proposals.push(proposal_id.clone());
            self.data::<Data>().member_proposals.insert(&caller, &proposals);
        } else {
            let proposals = vec![proposal_id.clone()];
            self.data::<Data>().member_proposals.insert(&caller, &proposals);
        }

        self.data::<Data>().vote.insert(&proposal_id.clone(),&vote);

        self.data::<Data>().proposal_id = proposal_id;

        self.emit_proposal_created_event(caller,proposal_id);

        Ok(())
    }
    
    default fn vote(&mut self, proposal_id: ProposalId, vote_cast: bool) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        if self.data::<Data>().vote.get(&proposal_id).is_none() {
            return Err(DaoError::ProposalDoesNotExist);
        }

        let mut vote = self.data::<Data>().vote.get(&proposal_id).unwrap();

        let pvote = self.data::<Data>().member_votes.get(&(caller.clone(),proposal_id)).is_some();

        if pvote == true {
            return Err(DaoError::MemberHasAlreadyVoted)
        }

        let now = Self::env().block_timestamp();

        if now > vote.end {
            return Err(DaoError::VotingPeriodExpired)
        }

        if vote_cast == true {
            vote.yes_votes += 1;
        } else {
            vote.no_votes += 1;
        }

        self.data::<Data>().vote.insert(&proposal_id.clone(),&vote);

        Ok(())
    }

    default fn finalize_vote(&mut self, proposal_id: ProposalId) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.data::<Data>().members.contains(&caller) {
            return Err(DaoError::MemberDoesNotExist)
        }

        if self.data::<Data>().vote.get(&proposal_id).is_none() {
            return Err(DaoError::ProposalDoesNotExist);
        }

        let mut vote = self.data::<Data>().vote.get(&proposal_id).unwrap();

        if vote.vote_status != VoteStatus::InProgress {
            return Err(DaoError::VoteNotAvailable)
        }

        let now = Self::env().block_timestamp();

        if now < vote.end {
            return Err(DaoError::VoteOngoing)
        }

        if vote.yes_votes + vote.no_votes < self.data::<Data>().quorum {
            vote.vote_status = VoteStatus::Failed;
            self.data::<Data>().vote.insert(&proposal_id.clone(),&vote);
        }

        if vote.yes_votes > vote.no_votes {
            vote.vote_status = VoteStatus::Passed;
            self.data::<Data>().vote.insert(&proposal_id.clone(),&vote);
        } else {
            vote.vote_status = VoteStatus::Failed;
            self.data::<Data>().vote.insert(&proposal_id.clone(),&vote);
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

        self.emit_project_created_event(caller,project_id);

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

    default fn create_task(&mut self,description: String,assignee: AccountId, reviewer: AccountId, duration: Timestamp, points: u32, priority: u8) -> Result<(),DaoError> {
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
            description: description,
            assignee: assignee.clone(),
            reviewer: reviewer,
            owner: caller.clone(),
            deadline: now + duration,
            points: points,
            priority: task_priority,
            status: TaskStatus::ToDo,
            review: String::from(""),
        };

        let task_id = self.data::<Data>().task_id.saturating_add(1);

        let member_tasks = self.data::<Data>().member_tasks.get(&assignee);

        if let Some(mut tasks) = member_tasks {
            tasks.push(task_id.clone());
            self.data::<Data>().member_tasks.insert(&assignee, &tasks);
        } else {
            let tasks = vec![task_id.clone()];
            self.data::<Data>().member_tasks.insert(&assignee, &tasks);
        }

        self.data::<Data>().task.insert(&task_id.clone(),&task);


        self.data::<Data>().task_id = task_id;

        self.emit_task_created_event(caller,task_id);

        Ok(())
    }

    default fn create_project_task(&mut self, project_id: ProjectId, description: String, assignee: AccountId, reviewer: AccountId, duration: Timestamp,
    points: u32, priority: u8) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if !self.is_project_member(project_id.clone(),caller.clone()) {
            return Err(DaoError::NotAProjectMember)
        }

        if !self.is_project_member(project_id.clone(),assignee.clone()) {
            return Err(DaoError::NotAProjectMember)
        }

        if !self.is_project_member(project_id.clone(),reviewer.clone()) {
            return Err(DaoError::NotAProjectMember)
        }

        let now = Self::env().block_timestamp();

        let deadline = now + duration;

        let task_priority = match priority {
            1 => TaskPriority::Low,
            2 => TaskPriority::Moderate,
            3 => TaskPriority::High,
            _ => return Err(DaoError::WrongTaskPriority)
        };

        let task_id = self.create_task_internal(description,caller.clone(),assignee.clone(),reviewer.clone(),deadline,task_priority,points);

        let project_tasks = self.data::<Data>().project_tasks.get(&project_id);

        if let Some(mut tasks) = project_tasks {
            tasks.push(task_id.clone());
            self.data::<Data>().project_tasks.insert(&project_id, &tasks);
        } else {
            let tasks = vec![task_id.clone()];
            self.data::<Data>().project_tasks.insert(&project_id, &tasks);
        }

        let member_tasks = self.data::<Data>().member_tasks.get(&assignee);

        if let Some(mut mtasks) = member_tasks {
            mtasks.push(task_id.clone());
            self.data::<Data>().member_tasks.insert(&assignee, &mtasks);
        } else {
            let mtasks = vec![task_id.clone()];
            self.data::<Data>().member_tasks.insert(&assignee, &mtasks);
        }

        self.emit_task_created_event(caller,task_id);

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

    default fn review_task(&mut self, task_id: TaskId, review: String, awarded_points: u32) -> Result<(),DaoError> {
        let caller = Self::env().caller();

        if self.data::<Data>().task.get(&task_id).is_none() {
            return Err(DaoError::TaskDoesNotExist);
        }

        let mut task = self.data::<Data>().task.get(&task_id).unwrap();

        if task.reviewer != caller {
            return Err(DaoError::IneligibleCaller)
        }

        task.status = TaskStatus::Done;
        task.review = review;

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

    default fn get_quorum(&self) -> u32 {
        self.data::<Data>().quorum
    }

    default fn get_number_of_members(&self) -> u32 {
        self.data::<Data>().member_id
    }

    default fn get_members(&self) -> Vec<AccountId> {
        self.data::<Data>().members.clone()
    }

    default fn get_project_members(&self,project_id: ProjectId) -> Vec<AccountId> {
        let vec1 = vec![];
        if project_id == 0 || project_id > self.data::<Data>().project_id {
            return vec1
        }
        let proj = self.data::<Data>().project_members.get(&project_id);

        if let Some(project) = proj {
            return project;
        } else {
            return vec1;
        }

    }

    default fn get_proposal_vote(&self,proposal_id: ProposalId) -> Vote {
        let vote = Vote {
            yes_votes: 0,
            no_votes: 0,
            start: 0,
            end: 0,
            vote_status: VoteStatus::NotAvailable,
        };
        if proposal_id == 0 || proposal_id > self.data::<Data>().proposal_id {
            return vote;
        }
        let vote2 = self.data::<Data>().vote.get(&proposal_id);

        if let Some(vec) = vote2 {
            return vec;
        } else {
            return vote;
        }
    }

    default fn get_current_vote_count(&self,proposal_id: ProposalId) -> (u32,u32) {
        let vcount = (0,0);
        if proposal_id == 0 || proposal_id > self.data::<Data>().proposal_id {
            return vcount;
        }
        let vote1 = self.data::<Data>().vote.get(&proposal_id);
        if let Some(vote) = vote1 {
            (vote.yes_votes,vote.no_votes)
        } else {
            vcount
        }
        
    }

    default fn get_number_of_projects(&self) -> u32 {
        self.data::<Data>().project_id
    }

    default fn get_number_of_tasks(&self) -> u32 {
        self.data::<Data>().task_id
    }

    default fn get_task(&self, task_id: TaskId) -> Task {
        let task = Task {
            description: String::from(""),
            assignee: ZERO_ADDRESS.into(),
            reviewer: ZERO_ADDRESS.into(),
            owner: ZERO_ADDRESS.into(),
            deadline: 0,
            points: 0,
            priority: TaskPriority::None,
            status: TaskStatus::ToDo,
            review: String::from(""),
        };
        if task_id == 0 || task_id > self.data::<Data>().task_id {
            return task;
        }

        let task1 = self.data::<Data>().task.get(&task_id);
        if let Some(task2) = task1 {
            return task2;
        } else {
            return task;
        }

    }

    default fn get_project_task_ids(&self,project_id: ProjectId) -> Vec<TaskId> {

        if project_id == 0 || project_id > self.data::<Data>().project_id {
            return vec![];
        }
        let proj = self.data::<Data>().project_tasks.get(&project_id);

        if let Some(proj1) = proj {
            return proj1;
        } else {
            return vec![];
        }
    }

    default fn get_member_points(&self, assignee: AccountId) -> u32 {
        let member_points = self.data::<Data>().member_points.get(&assignee);

        if let Some(points) = member_points {
            return points;
        } else {
            return 0;
        }
    }

    default fn get_member_task_ids(&self, assignee:AccountId) -> Vec<TaskId> {
        let member_tasks = self.data::<Data>().member_tasks.get(&assignee);

        if let Some(task_ids) = member_tasks {
            return task_ids;
        } else {
            return vec![];
        }
    }

    default fn get_member_proposal_ids(&self, assignee:AccountId) -> Vec<ProposalId> {
        let member_proposals = self.data::<Data>().member_proposals.get(&assignee);

        if let Some(proposal_ids) = member_proposals {
            return proposal_ids;
        } else {
            return vec![];
        }
    }

    default fn get_project(&self,project_id: ProjectId) -> Project {
        let project = Project {
            creator: ZERO_ADDRESS.into(),
            description: String::from(""),
        };
        if project_id == 0 || project_id > self.data::<Data>().project_id {
            return project;
        }
        let proj = self.data::<Data>().project.get(&project_id);

        if let Some(proj1) = proj {
            return proj1;
        } else {
            return project;
        }
    }

    default fn get_proposal(&self,proposal_id: ProposalId) -> Proposal {
        let proposal = Proposal {
            creator: ZERO_ADDRESS.into(),
            description: String::from(""),
        };
        if proposal_id == 0 || proposal_id > self.data::<Data>().proposal_id {
            return proposal;
        }
        let proj = self.data::<Data>().proposal.get(&proposal_id);

        if let Some(proj1) = proj {
            return proj1;
        } else {
            return proposal;
        }
    }

    default fn get_number_of_project_tasks(&self,project_id: ProjectId) -> u32 {
        let vec1 = self.data::<Data>().project_tasks.get(&project_id);//.len().try_into().unwrap()
        match vec1 {
            Some(vec1) => return vec1.len().try_into().unwrap(),
            None => return 0,
        }
    }

    default fn get_number_of_proposals(&self) -> u32 {
        self.data::<Data>().proposal_id
    }

}

impl<T> DaoEvents for T
where
    T: Storage<Data>,
{
    default fn emit_member_added_event(
        &self,
        _member: AccountId,
        _member_id: u32,
    ) {
    }

    default fn emit_project_created_event(
        &self,
        _member: AccountId,
        _project_id: u32,
    ) {
    }

    default fn emit_proposal_created_event(
        &self,
        _member: AccountId,
        _proposal_id: u32,
    ) {
    }

    default fn emit_task_created_event(
        &self,
        _member: AccountId,
        _task_id: u32,
    ) {
    }
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn is_eligible(&self,account: AccountId) -> bool {
        let address = self.data::<Data>().token;
        let balance = PSP34Ref::balance_of(&address, account.clone());

        if balance > 0 {
            return true;
        } else {
            return false;
        }
    }

    default fn is_project_member(&self,project_id: ProjectId,account: AccountId) -> bool {

        let project_members = self.data::<Data>().project_members.get(&project_id);

        if let Some(members) = project_members {
            if members.contains(&account) {
                return true;
            } else {
                return false;
            }
            
        } else {
            return false;
        }
    }

    default fn create_task_internal(&mut self,description: String, caller: AccountId,assignee: AccountId, reviewer: AccountId, deadline: Timestamp,
         task_priority: TaskPriority, points: u32) -> TaskId {
        
        let task = Task {
            description: description,
            assignee: assignee,
            reviewer: reviewer,
            owner: caller.clone(),
            deadline: deadline,
            points: points,
            priority: task_priority,
            status: TaskStatus::ToDo,
            review: String::from(""),
        };

        let task_id = self.data::<Data>().task_id.saturating_add(1);

        self.data::<Data>().task.insert(&task_id.clone(),&task);


        self.data::<Data>().task_id = task_id;

        task_id
    }

}