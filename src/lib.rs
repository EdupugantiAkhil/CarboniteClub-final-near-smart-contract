use near_sdk::borsh::{self,BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::json_types::Base64VecU8;
use near_sdk::collections::{UnorderedMap, UnorderedSet, LazyOption, LookupMap};

use near_sdk::{
    near_bindgen, env, require, AccountId,BorshStorageKey, PanicOnDefault,CryptoHash,Timestamp
};

use std::collections::HashSet;

mod task;
mod company;
mod metadata;
mod utils;

pub use crate::task::*;
pub use crate::company::*;
pub use crate::metadata::*;
pub use crate::utils::*;

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    TokenByAccountId,
    TasksCompletedPerAccount,
    TasksCompletedPerAccountInner {account_id_hash: CryptoHash},
    TasksByCompany,
    TasksByCompanyInner {task_id_hash: CryptoHash},
    TaskById,
    RecognisedSkills,
    WhitelistedCompanies,
    ApprovedFTTokens,
    NFTContractMetadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract{
    // owner of the contract
    pub owner_id: AccountId,

    // keeps track of metadata of carbonite NFT for a given account
    pub tokens_by_account_id: UnorderedMap<AccountId,TokenMetadata>,

    // keeps track of tasks that are completed and verified for a given account
    pub tasks_completed_per_account: LookupMap<AccountId,UnorderedSet<TaskId>>,

    // keeps track of all tasks that are given for a given company
    pub tasks_by_company: LookupMap<AccountId,UnorderedSet<TaskId>>,

    // keeps track of task metadata for a given task ID
    pub task_metadata_by_id: UnorderedMap<TaskId,Task>,

    // keeps track of all the skills that are recognised by carbonite community
    pub recognised_skills: UnorderedSet<Skills>,

    // keeps track of whitelisted companies that are verified to be genuine
    pub whitelisted_companies: UnorderedMap<AccountId,Company>,

    // keeps track of approved tokens that the companies can use for paying
    pub approved_ft_tokens: UnorderedSet<AccountId>,

    // keeps track of metadata of the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        unimplemented!();
    }
}