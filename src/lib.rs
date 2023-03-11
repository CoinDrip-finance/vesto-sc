#![no_std]

use storage::{InputStream, VestingSchedule};

use crate::errors::{ERR_STREAM_ONLY_FUNGIBLE, ERR_ONLY_OWNER_CAN_MODIFY, ERR_STREAM_DEPOSIT_TOO_BIG};

multiversx_sc::imports!();

mod storage;
mod errors;
mod events;

#[multiversx_sc::contract]
pub trait VestoContract: 
    storage::StorageModule
    + events::EventsModule {

    #[proxy]
    fn coindrip_proxy(&self, sc_address: ManagedAddress) -> coindrip::Proxy<Self::Api>;

    #[init]
    fn init(
        &self,
        coindrip_address: ManagedAddress
    ) {
        self.coindrip_address().set_if_empty(coindrip_address);
        self.last_vesting_schedule_id().set_if_empty(0u64);
    }

    #[payable("*")]
    #[endpoint(createVestingSchedule)]
    fn create_vesting_schedule(
        &self, 
        vesting_schedule_list: ManagedVec<InputStream<Self::Api>>
    ) {
        let (token_identifier, token_nonce, token_amount) = self.call_value().egld_or_single_esdt().into_tuple();
        let caller = self.blockchain().get_caller();

        require!(token_nonce == 0, ERR_STREAM_ONLY_FUNGIBLE);

        let mut to_be_vested = BigUint::zero();

        let vesting_schedule_id = self.last_vesting_schedule_id().get();

        for vesting_item in &vesting_schedule_list {
            let payment = EgldOrEsdtTokenPayment::new(
                token_identifier.clone(),
                token_nonce,
                vesting_item.deposit.clone()
            );

            let stream_id = self.coindrip_proxy(self.coindrip_address().get())
            .create_stream(vesting_item.recipient.clone(), vesting_item.start_time, vesting_item.end_time, true)
            .with_egld_or_single_esdt_transfer(payment)
            .execute_on_dest_context::<u64>();

            to_be_vested += vesting_item.deposit.clone();

            self.add_user_to_vesting_schedule_event(vesting_schedule_id, &vesting_item.recipient, &vesting_item.deposit, stream_id, vesting_item.start_time, vesting_item.end_time);
        }

        let new_vesting = VestingSchedule {
            owner: caller.clone(),
            token_identifier: token_identifier.clone(),
            token_nonce,
            deposit: token_amount.clone(),
            to_be_vested
        };

        self.vesting_schedule_list(vesting_schedule_id).set(new_vesting);

        self.last_vesting_schedule_id().set(vesting_schedule_id + 1);

        self.create_vesting_schedule_event(vesting_schedule_id, &caller, &token_identifier, token_nonce, &token_amount);
    }

    #[endpoint(addUserToVestingSchedule)]
    fn add_user_to_vesting_schedule(
        &self, 
        vesting_schedule_id: u64,
        new_vesting_item: InputStream<Self::Api>
    ) {
        let mut vesting_schedule = self.vesting_schedule_list(vesting_schedule_id).get();
        let caller = self.blockchain().get_caller();

        require!(caller == vesting_schedule.owner, ERR_ONLY_OWNER_CAN_MODIFY);

        require!(vesting_schedule.to_be_vested.clone() + new_vesting_item.deposit.clone() <= vesting_schedule.deposit, ERR_STREAM_DEPOSIT_TOO_BIG);

        let payment = EgldOrEsdtTokenPayment::new(
            vesting_schedule.token_identifier.clone(),
            vesting_schedule.token_nonce,
            new_vesting_item.deposit.clone()
        );

        let stream_id = self.coindrip_proxy(self.coindrip_address().get())
        .create_stream(new_vesting_item.recipient.clone(), new_vesting_item.start_time, new_vesting_item.end_time, true)
        .with_egld_or_single_esdt_transfer(payment)
        .execute_on_dest_context::<u64>();

        vesting_schedule.to_be_vested += new_vesting_item.deposit.clone();

        self.vesting_schedule_list(vesting_schedule_id).set(vesting_schedule);

        self.add_user_to_vesting_schedule_event(vesting_schedule_id, &new_vesting_item.recipient, &new_vesting_item.deposit, stream_id, new_vesting_item.start_time, new_vesting_item.end_time);
    }
}
