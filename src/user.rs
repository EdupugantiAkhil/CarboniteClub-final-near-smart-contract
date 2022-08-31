use crate::*;

#[near_bindgen]
impl Contract {
    // reciever_id is expected to be anonymous_name.carbonite_contract_id
    // will mint an nft after creating the sub a/c
    // signer a/c public key will be used to create the new a/c if no explicit public key (we think ed25519 curve should be used for a/c creation not sure) is passed
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        title: String,
        description: Option<String>,
        public_key: Option<PublicKey>,
    ) {
        assert_valid_carbonite_user_account(receiver_id.as_str());

        Promise::new(receiver_id.clone())
            .create_account()
            .transfer(BASE_STORAGE_COST)
            .add_full_access_key(public_key.unwrap_or_else(|| env::signer_account_pk()));

        let token_metadata = TokenMetadata::new(title, description);

        // internal add token_to_owner
        require!(
            self.tokens_by_account_id
                .insert(&receiver_id, &token_metadata)
                .is_none()
                == true,
            "Account ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );

        // while onboarding users, for a fixed size of title and description appropriate amount of allowance will be given to their funciton access key
        // and appropriate amount of near to cover storage costs
        // for standarisation purpose later a mint_event will be emitted
        // Add a gas check to ensure sub account creation and the full execution if account creation does not revert on panic
        todo!();
    }
}
