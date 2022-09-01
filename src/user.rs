use crate::*;

#[near_bindgen]
impl Contract {
    /// reciever_id is expected to be anonymous_name.carbonite_contract_id
    /// will mint an nft after creating the sub a/c
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        title: String,
        description: Option<String>,
        public_key: PublicKey,
    ) {
        let initial_storage = env::storage_usage();

        assert_valid_carbonite_user_account(receiver_id.as_str());

        create_sub_account(receiver_id.clone(), public_key);

        let token_metadata = TokenMetadata::new_default(title, description);

        self.internal_add_token_to_owner(&receiver_id, &token_metadata);

        let storage_used = env::storage_usage() - initial_storage;

        refund_deposit(storage_used);

        // while onboarding users, for a fixed size of title and description appropriate amount of allowance will be given to their funciton access key
        // and appropriate amount of near to cover storage costs
        // for standarisation purpose later a mint_event will be emitted
        // Add a gas check to ensure sub account creation and the full execution if account creation does not revert on panic
        // think of making it a batch mint function
        todo!();
    }
}

/// asserts that passed account ID is exactly of form valid_username.carbonite.near
pub(crate) fn assert_valid_carbonite_user_account(account_id: &str) {
    if let Some((username, carbonite_contract_id)) = account_id.split_once(".") {
        require!(
            username
                .bytes()
                .into_iter()
                .all(|c| matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')),
            "Invalid username passed"
        );

        require!(
            carbonite_contract_id == env::current_account_id().as_str(),
            "Invalid account ID passed"
        );
    } else {
        env::panic_str("Invalid account ID passed")
    }
}
