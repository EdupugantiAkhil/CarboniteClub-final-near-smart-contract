use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskType {
    InviteOnly {
        invited_accounts: HashSet<AccountId>,
        valid_till: Timestamp,
    }, // keeps track of invited accounts if an invite only project and validity date till if which if no-one accepts then company can claim refund
    ForEveryone, // this task can be taken up by anyone the company has the choice to select the winner
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskDetails {
    pub title: String,
    pub description: String,     // short description about the task
    pub required_skills: String, // required skills for the task in a comma seperated format
    pub task_type: TaskType,
    pub reference: String, // URL to an off-chain JSON file with more info, preferably a decentralised storage in encrypted format
    pub reference_hash: Base64VecU8, // Base64-encoded sha256 hash of Jencrypted JSON file itself from reference field
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub task_details: TaskDetails,
    pub company: AccountId,  // account ID of the company giving this task
    pub deadline: Timestamp, // if task is not completed till this then company can claim refund
    pub person_assigned: Option<AccountId>, // person assigned or person thst accepted the invite for task in an invite only task
    pub ft_contract_id: AccountId,          // contract ID of approved token used to pay
    pub reward: u64, // reward amount in smallest unit of tokens, Eg: for near it will be yoctoNEAR
    pub submission_reference: HashMap<AccountId, String>, // keeps track of user_account and their submission link (preferably a decentralised storage in encrypted format)
    pub submission_reference_hash: HashMap<AccountId, String>,
}
