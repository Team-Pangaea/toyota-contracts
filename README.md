# toyota-contracts

### DAO API

#### PROJECTS

Create a Project
```
fn create_project(&mut self, description: String) -> Result<(),DaoError>;
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

Finalize Vote (proposal Id) - Any DAO member can finalzie vote when the time has expired.
```
fn finalize_vote(&mut self, proposal_id: ProposalId) -> Result<(),DaoError>;
```

#### TASKS

#### MEMBERS


### DAOMANAGER API

Get the total Number of DAOs
```
fn get_number_of_daos(&self) -> DaoId
```

Add a DAO ( Members only function)
```
fn add_dao(&mut self, dao: AccountId) -> Result<(),DaoManagerError>
```

Register Account as a Member
```
fn register(&mut self, account: AccountId) -> Result<(),DaoManagerError>
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
