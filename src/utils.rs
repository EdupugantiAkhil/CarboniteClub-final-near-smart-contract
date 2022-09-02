use crate::*;

/// returns SHA-256 hash of the passed account ID
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

/// creates sub account provide with BASE_STORAGE_COST for a given account ID and public key
pub(crate) fn create_sub_account(account_id: AccountId, public_key: PublicKey) {
    Promise::new(account_id.clone())
        .create_account()
        .transfer(BASE_STORAGE_COST)
        .add_full_access_key(public_key);
}

/// refunds excess deposit attached to predecessor_account_id and panic if attached deposit is not enough to cover given storage_used in bytes
pub(crate) fn refund_excess_deposit(storage_used: u64) {
    let storage_cost = env::storage_byte_cost() * (storage_used as u128);

    let refund_amount = env::attached_deposit()
        .checked_sub(storage_cost)
        .unwrap_or_else(|| env::panic_str("attached deposit was not enough"));

    if refund_amount > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund_amount);
    }
}

impl Contract {
    pub fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only Contract Owner can call this method"
        );
    }
}
