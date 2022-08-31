use crate::*;

impl Contract {
    /// adds given token_metadata and associated a/c ID to tokens_by_account_id, panics if given a/c ID was already present in the collection
    pub fn internal_add_token_to_owner(
        &mut self,
        receiver_id: &AccountId,
        token_metadata: &TokenMetadata,
    ) {
        require!(
            self.tokens_by_account_id
                .insert(receiver_id, token_metadata)
                .is_none()
                == true,
            "Account ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );
    }
}
