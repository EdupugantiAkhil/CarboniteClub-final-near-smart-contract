use crate::*;

// returns SHA-256 hash of the passed account ID
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// asserts that passed account ID is exactly of form valid_username.carbonite.near
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
        env::panic_str("Invalid Account Id passed")
    }
}

impl Contract {
    fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only Contract Owner can call this method"
        );
    }
}
