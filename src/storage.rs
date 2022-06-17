elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("initialClaim")]
    fn initial_claim(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("currentClaim")]
    fn current_claim(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastClaimDate")]
    fn last_claim_date(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("rewardToken")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("rate")]
    fn rate(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("unvestStart")]
    fn unvest_start(&self) -> SingleValueMapper<u64>;
}
