use crate::*;

// company account ID will be suffixed with -Co whereas users can't have _ in their name

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Company {
    pub name: String,
    pub icon: String,             // Data URL of company logo
    pub industries: String, // various industries (comma seperated) in which company is working
    pub description: String, // short description about the company
    pub location: Option<String>, // None if company is remote or else represents headquarter location
    pub reference: String,        // website url of the company
}

#[near_bindgen]
impl Contract {
    /// company only method to edit company details
    #[payable]
    pub fn edit_company_details(&mut self, new_company_details: Company) {
        let initial_storage = env::storage_usage();

        let company_id = env::predecessor_account_id();

        require!(
            self.whitelisted_companies
                .insert(&company_id, &new_company_details)
                .is_some(),
            "invalid company"
        );

        let final_storage = env::storage_usage();

        let storage_used = final_storage.abs_diff(initial_storage);

        if final_storage > initial_storage {
            refund_deposit(storage_used);
        } else if final_storage < initial_storage {
            let refund_amount = storage_used as u128 * env::storage_byte_cost();
            Promise::new(company_id).transfer(refund_amount);
        }
    }
}

/// asserts that passed account ID is exactly of form company_name-Co.carbonite.near
pub(crate) fn assert_valid_carbonite_company_account_pattern(account_id: &str) {
    if let Some((mut company_name, carbonite_contract_id)) = account_id.split_once(".") {
        require!(
            company_name.ends_with("-Co"),
            "Invlalid company name passed"
        );

        (company_name, _) = company_name.split_once("-").unwrap();

        require!(
            company_name
                .bytes()
                .into_iter()
                .all(|c| matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')),
            "Invalid company name passed"
        );

        require!(
            carbonite_contract_id == env::current_account_id().as_str(),
            "Invalid account ID passed"
        );
    } else {
        env::panic_str("Invalid account ID passed")
    }
}
