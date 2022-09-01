use crate::*;

impl Contract {
    /// adds given token_metadata and associated a/c ID to tokens_by_account_id, panics if given a/c ID was already present in the collection
    pub fn internal_add_token_to_owner(
        &mut self,
        owner_id: &AccountId,
        token_metadata: &TokenMetadata,
    ) {
        require!(
            self.tokens_by_account_id
                .insert(owner_id, token_metadata)
                .is_none()
                == true,
            "account ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );
    }

    /// adds given company and associated a/c ID to whitelisted_companies, panics if given a/c ID was already present in the collection
    pub fn internal_add_company_to_whitelisted_companies(
        &mut self,
        company_id: &AccountId,
        company: &Company,
    ) {
        require!(
            self.whitelisted_companies
                .insert(company_id, company)
                .is_none()
                == true,
            "company ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );
    }
}
