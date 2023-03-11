multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, ManagedVecItem, NestedDecode)]
pub struct InputStream<M: ManagedTypeApi> {
    pub recipient: ManagedAddress<M>,
    pub deposit: BigUint<M>,
    pub start_time: u64,
    pub end_time: u64
} 

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct VestingSchedule<M: ManagedTypeApi> {
    pub owner: ManagedAddress<M>,
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub deposit: BigUint<M>,
    pub to_be_vested: BigUint<M>
} 

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("coinDripAddress")]
    fn coindrip_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("lastVestingScheduleId")]
    fn last_vesting_schedule_id(&self) -> SingleValueMapper<u64>;
    #[storage_mapper("vestingScheduleList")]
    fn vesting_schedule_list(&self, vesting_schedule_id: u64) -> SingleValueMapper<VestingSchedule<Self::Api>>;
}