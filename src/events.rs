multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("createVestingSchedule")]
    fn create_vesting_schedule_event(
        &self,
        #[indexed] vesting_schedule_id: u64,
        #[indexed] owner: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] deposit: &BigUint
    );

    #[event("addUserToVestingSchedule")]
    fn add_user_to_vesting_schedule_event(
        &self,
        #[indexed] vesting_schedule_id: u64,
        #[indexed] address: &ManagedAddress,
        #[indexed] amount: &BigUint,
        #[indexed] stream_id: u64,
        #[indexed] start_date: u64,
        #[indexed] end_date: u64,
    );
}  