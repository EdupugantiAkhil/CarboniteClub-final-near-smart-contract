use crate::*;

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

