use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Company{
    pub name: String,
    pub icon: String,                   // Data URL of company logo
    pub industries: String,             // various industries (space seperated) in which company is working
    pub description: String,            // short description about the company
    pub location: Option<String>,       // None if company is remote or else represents headquarter location
    pub reference: String,              // website url of the company
}