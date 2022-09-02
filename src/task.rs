use near_sdk::borsh::de;

use crate::*;

/// TaskId = company_name.task_name      company account_id = company_name.carbonite.near
pub type TaskId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskType {
    InviteOnly {
        invited_accounts: HashSet<AccountId>, // should be ideally be 3
        valid_till: Timestamp,                // unix epoch in ms
    }, // keeps track of invited accounts if an invite only project and validity date till if which if no-one accepts then company can claim refund
    ForEveryone, // this task can be taken up by anyone the company has the choice to select the winner
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskDetails {
    pub title: String,
    pub description: String,     // short description about the task
    pub required_skills: Skills, // required skills for the task in a comma seperated format
    pub task_type: TaskType,
    pub reference: String, // URL to an off-chain JSON file with more info, preferably a decentralised storage in encrypted format
    pub reference_hash: Base64VecU8, // Base64-encoded sha256 hash of Jencrypted JSON file itself from reference field
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Submission {
    pub submission_reference: String, // link to the decentralised submitted documents in encrypted format (preferrably)
    pub submission_reference_hash: Base64VecU8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub task_details: TaskDetails,
    pub company_id: AccountId, // account ID of the company giving this task
    pub deadline: Timestamp, // if task is not completed till this (unix epoch in ms) then company can claim refund
    pub person_assigned: Option<AccountId>, // person assigned or person thst accepted the invite for task in an invite only task
    pub ft_contract_id: AccountId,          // contract ID of approved token used to pay
    pub reward: Balance, // reward amount in smallest unit of tokens, Eg: for near it will be yoctoNEAR
    pub submissions_by_account_id: HashMap<AccountId, Submission>, // keeps track of user_account and their submission
}

impl TaskDetails {
    /// assert that task_details are valid else panic
    pub fn assert_valid_task_details(&self) {
        require!(
            self.reference_hash.0.len() == 32,
            "hash should be 32 bytes long"
        )
    }
}

impl Task {
    /// creates task struct out of details given and also validates if task details are valid
    pub fn new(
        task_details: TaskDetails,
        company_id: AccountId,
        deadline: Timestamp,
        ft_contract_id: AccountId,
        reward: u128,
    ) -> Self {
        task_details.assert_valid_task_details();

        // asserting deadline is after current time is not necessary as even if it's wrong it won't casue any harm

        Self {
            task_details,
            company_id,
            deadline,
            person_assigned: None,
            ft_contract_id,
            reward,
            submissions_by_account_id: Default::default(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_task_in_near_token(
        &mut self,
        task_id: TaskId,
        task_details: TaskDetails,
        deadline: Timestamp,
        reward: Balance,
    ) {
        let initial_storage = env::storage_usage();

        let company_id = env::predecessor_account_id();

        self.assert_whitelisted_company(&company_id);

        let near_contract_id = AccountId::new_unchecked("near".to_string());

        let task = Task::new(
            task_details,
            company_id.clone(),
            deadline,
            near_contract_id,
            reward,
        );
        self.internal_add_tasks_to_company(&company_id, &task_id);

        self.task_metadata_by_id.insert(&task_id, &task);

        let storage_used = env::storage_usage() - initial_storage;
        let storage_cost = storage_used as u128 * env::storage_byte_cost();

        let refund_amount = env::attached_deposit() - (storage_cost + reward);

        if refund_amount > 0 {
            Promise::new(company_id).transfer(refund_amount);
        } else {
            env::panic_str("attached deposit was not enough");
        }
    }

    pub fn extend_deadline(new_deadline: Timestamp) {}

    pub fn submit_task(task_id: TaskId, submission: Submission) {}
}
