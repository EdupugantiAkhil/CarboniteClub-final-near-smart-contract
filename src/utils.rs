use crate::*;

#[macro_export]
macro_rules! impl_assert_valid_metadata_fn_for{
    ($($type:ty),+) => {
            $(impl $type{
                    pub fn assert_valid_metadata(&self) {
                        require!(self.reference_hash.0.len() == 32, "Hash has to be 32 bytes");
                    }
            })*
    };
}

// asserts that passed account ID is exactly of form username.carbonite.near
pub(crate) fn assert_valid_carbonite_account(){
    unimplemented!();
}

impl Contract{
    fn assert_owner(&self){
        require!(env::predecessor_account_id() == self.owner_id,
            "Only Contract Owner can call this method"
        );
    }
}

