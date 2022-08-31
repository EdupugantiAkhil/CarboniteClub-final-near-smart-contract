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

        Promise::new(receiver_id.clone())
            .create_account()
            .transfer(BASE_STORAGE_COST)
            .add_full_access_key(public_key);

        let token_metadata = TokenMetadata::new_default(title, description);

        self.internal_add_token_to_owner(&receiver_id, &token_metadata);

        let storage_used = env::storage_usage() - initial_storage;

        refund_deposit(storage_used);

        // while onboarding users, for a fixed size of title and description appropriate amount of allowance will be given to their funciton access key
        // and appropriate amount of near to cover storage costs
        // for standarisation purpose later a mint_event will be emitted
        // Add a gas check to ensure sub account creation and the full execution if account creation does not revert on panic
        todo!();
    }
}
