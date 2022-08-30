use crate::*;

pub type Skills = String;

// TaskId = company_name.task_name      company account_id = company_name.carbonite.near
pub type TaskId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: String,                   // Data URL
    pub base_uri: String,               // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: String,              // URL to a JSON file with more info
    pub reference_hash: Base64VecU8,    // Base64-encoded sha256 hash of JSON from reference field
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: String,
    pub description: Option<String>,            // free-form description
    pub media: String,                          // URL to associated media stored on decentralised storage platform
    pub media_hash: Base64VecU8,
    pub issued_at: Timestamp,                   // When token was issued or minted, Unix epoch in milliseconds
    pub updated_at: Option<Timestamp>,
    pub extra: Option<String>,                  // anything extra the NFT wants to store on-chain. Can be stringified JSON. for our purpose it can be achievement
    pub carbonite_metadata: CarboniteMetdata,
    pub reference: String,                      // URL to an off-chain JSON file with more info
    pub reference_hash: Base64VecU8,            // Base64-encoded sha256 hash of JSON from reference field
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CarboniteMetdata{
    pub xp: u16,
    pub current_working_task_count: u8,
    pub total_tasks_completed: u16,
    pub total_bounty_earned: u32,
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

impl_assert_valid_metadata_fn_for!(NFTContractMetadata,TokenMetadata,Task);