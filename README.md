# toyota-contracts

There are two contracts

1. DAO Contract : It is the contract where all the DAO action happens.

2. DaoManager Contract : It is used to manage the DAOs and membership od the organization.

There is also a NFT Token contract that is used to manage membership. All the contracts are instantiated with this token contract address and to be a member one must have one of these NFT tokens. The assumption is that the Organization/Company will distribute it to its current/future employees and contributors. The token contracts are here -> https://github.com/Team-Pangaea/custom_mint . The front-end of this app is designed to detect the presence of this token and for this demo , the user will be prompted to mint one. If you want to directly mint the token, it should be minted as following

```
mint(&mut self, to: AccountId, token_uri: String, daomanagerContract: AccountId) -> Result<(), PSP34Error>;
```


The two contracts in this repo can be build with

```
cargo +nightly contract build
```

Unit Tests can be run with

```
cargo +nightly test
```

#### Contract Addresses (Shibuya Testnet)

```
DaoManager : ZdMvVgENtf5VhWF62eGV9nFjmScx3zZTteDjzGN74hyAUQR
Dao : WhRgUct8GRyMezkCSAjQj4Y6tJ5apVuHewNNtfvPFS8uymq (one of many)
NFT Token (Customized PSP34) : Xc5P6n26FkToLvLzJJQJcewa3doT2RctHMkGsoXCo2hCJbx
```


### Overview

Both DaoManager and Dao contracts are instantiated with NFT token (address Xc5P6n26FkToLvLzJJQJcewa3doT2RctHMkGsoXCo2hCJbx Shibuya Testnet)

1. DaoManager Contract : Any Token holder ( In this example anyone who has minted a token from the given NFT contract address ) can register for membership here. Members can add DAOs and join any DAO.
2. DAO contracts: Each DAO contract corresponds to a separate DAO. Members can create their own DAOs with its own contract as long as it uses the same token address as the daomanager. Members can join force, create projects, proposals, vote on proposals, create and accomplish tasks.


### DAO API

#### PROJECTS

Create a Project
```
fn create_project(&mut self, description: String) -> Result<(),DaoError>;
```

Join an existing project
```
fn join_project(&mut self, project_id: ProjectId ) -> Result<(),DaoError>
```

Get the vector of addresses of all project members
```
fn get_project_members(&self,project_id: ProjectId) -> Vec<AccountId>
```

Get number of projects
```
fn get_number_of_projects(&self) -> u32
```

Get the project by project ID
```
fn get_project(&self,project_id: ProjectId) -> Project
```

Create Task for a Project
```
fn create_project_task(&mut self, project_id: ProjectId, assignee: AccountId, reviewer: AccountId, duration: Timestamp,
        points: u32, priority: u8) -> Result<(),DaoError>
```

Get number of tasks for a project
```
fn get_number_of_project_tasks(&self,project_id: ProjectId) -> u32
```

Get the full list of all Tasks for a project
```
fn get_project_task_ids(&self,project_id: ProjectId) -> Vec<TaskId>
```



#### PROPOSALS

Create a Proposal (description, duration in milliseconds)
```
fn create_proposal(&mut self,description: String, duration: Timestamp) -> Result<(),DaoError>;
```

Vote on the Proposal (proposal Id, true for yes/false for no vote)
```
fn vote(&mut self, proposal_id: ProposalId, vote_cast: bool) -> Result<(),DaoError>;
```

Finalize Vote (proposal Id) - Any DAO member can finalize vote when the time has expired.
```
fn finalize_vote(&mut self, proposal_id: ProposalId) -> Result<(),DaoError>;
```

Get the Proposal Struct for a proposal id
```
fn get_proposal(&self,proposal_id: ProposalId) -> Proposal;
```

Get the Vote struct for a proposal id
```
fn get_proposal_vote(&self,proposal_id: ProposalId) -> Vote
```

Get Total Number of Proposals
```
fn get_number_of_proposals(&self) -> u32
```

Get Current Vote Count for a proposal id returns (yes votes, no votes)
```
fn get_current_vote_count(&self,proposal_id: ProposalId) -> (u32,u32)
```

#### TASKS

Create a Task : duration is in milliseconds, priority takes only 3 values (1,2,3) corresponding to (low, moderate,high) priority respectively.
```
fn create_task(&mut self,assignee: AccountId, reviewer: AccountId, duration: Timestamp, points: u32, priority: u8) -> Result<(),DaoError>
```

Assignee starts a task, changes status to In Progress
```
fn start_task(&mut self, task_id: TaskId) -> Result<(),DaoError>
```

Assignee finishes a task and changes the status to UnderReview
```
fn submit_task(&mut self, task_id: TaskId) -> Result<(),DaoError>
```

Reviewer reviews the task and allocates points to the assignee
```
fn review_task(&mut self, task_id: TaskId, review: String, awarded_points: u32) -> Result<(),DaoError>
```

Get total number of Tasks
```
fn get_number_of_tasks(&self) -> u32
```

Get the Task struct for a given Task Id
```
fn get_task(&self, task_id: TaskId) -> Task
```

#### MEMBERS

(Temporary Admin function) : Add member
```
fn add_member(&mut self,address: AccountId) -> Result<(),DaoError>
```

Join DAO : Must have NFT Token
```
fn join_dao(&mut self) -> Result<(),DaoError>
```

Get NFT Token Address
```
fn get_token_address(&self) -> AccountId
```

Get Quorum (Set to zero for this demo)
```
fn get_quorum(&self) -> u32
```

Vector containing member addresses
```
fn get_members(&self) -> Vec<AccountId>
```

Get number of members
```
fn get_number_of_members(&self) -> u32
```

Get member points
```
fn get_member_points(&self, assignee: AccountId) -> u32
```

Get member Task Ids vector
```
fn get_member_task_ids(&self, assignee:AccountId) -> Vec<TaskId>
```

Get Member Proposal Ids vector
```
fn get_member_proposal_ids(&self, assignee:AccountId) -> Vec<ProposalId>
```





### DAOMANAGER API

Get the total Number of DAOs
```
fn get_number_of_daos(&self) -> DaoId
```

Add DAO
```
fn add_dao(&mut self, dao: AccountId) -> Result<(),DaoManagerError>;
```

Register Account as a Member
```
fn register(&mut self) -> Result<(),DaoManagerError>
```

Get NFT Token Address
```
fn get_token(&self) -> AccountId
```

Get vector containing the account Id of all the DAOs
```
fn get_daos(&self) -> Vec<AccountId>
```

Get vector containing the account addresses of all the registered members
```
fn get_members(&self) -> Vec<AccountId>
```

Check membership
```
fn check_membership(&self,account: AccountId) -> bool
```
