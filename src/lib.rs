#![no_std]

elrond_wasm::imports!();

pub mod storage;
pub mod views;

const ONE_DAY: u64 = 24 * 60 * 60;
const ONE_MONTH: u64 = 30 * ONE_DAY;

#[elrond_wasm::contract]
pub trait ClaimsContract: storage::StorageModule + views::ViewsModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setRewardToken)]
    fn set_reward_token(&self, token: TokenIdentifier) {
        require!(self.reward_token().is_empty(), "Reward token already set");
        self.reward_token().set(token);
    }

    #[only_owner]
    #[endpoint(setUnvestStart)]
    fn set_unvest_start(&self, unvest_start: u64) {
        require!(self.unvest_start().is_empty(), "Unvest start already set");
        self.unvest_start().set(unvest_start);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(addClaim)]
    fn add_claim(&self, address: &ManagedAddress, rate: BigUint) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        require!(
            payment_token == self.reward_token().get(),
            "Payment token must be the same as the reward token"
        );
        require!(
            self.initial_claim(address).is_empty(),
            "Claim already exists"
        );
        require!(
            payment_amount > BigUint::zero(),
            "Must add more than 0 tokens"
        );
        self.initial_claim(address).set(payment_amount.clone());
        self.current_claim(address).set(payment_amount);
        self.last_claim_date(address)
            .set(self.unvest_start().get() - ONE_MONTH);
        self.rate(address).set(rate * BigUint::from(100u64));
    }

    #[endpoint(claim)]
    fn harvest_claim(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let unvest_start = self.unvest_start().get();
        require!(
            current_timestamp >= unvest_start,
            "Unvesting not started yet"
        );
        let initial_claim = self.initial_claim(&caller);
        let current_claim = self.current_claim(&caller);
        require!(!initial_claim.is_empty(), "No claim to harvest");
        require!(
            current_claim.get() > BigUint::from(0u64),
            "No claim to harvest"
        );
        let last_claim_date = self.last_claim_date(&caller).get();
        let time_of_harvest = current_timestamp - last_claim_date;
        let months_to_claim = time_of_harvest / ONE_MONTH;
        require!(months_to_claim > 0u64, "No claim to harvest");

        let mut harvest_amount =
            initial_claim.get() * self.rate(&caller).get() / BigUint::from(10000u64);
        if harvest_amount > current_claim.get() {
            harvest_amount = current_claim.get();
            current_claim.set(BigUint::zero());
        } else {
            current_claim.set(&current_claim.get() - &harvest_amount);
        }
        self.last_claim_date(&caller).set(current_timestamp);
        self.send().direct(
            &caller,
            &self.reward_token().get(),
            0u64,
            &harvest_amount,
            &[],
        );
    }
}
