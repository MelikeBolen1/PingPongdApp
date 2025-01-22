#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait PingPong {
    #[init]
    fn init(&self, ping_amount: BigUint, duration_in_seconds: u64) {
        self.ping_amount().set(&ping_amount);
        self.duration_in_seconds().set(duration_in_seconds);
    }

    // Only accept the exact ping amount
    #[payable("EGLD")]
    #[endpoint]
    fn ping(&self) {
        let payment = self.call_value().egld_value();
        require!(
            payment == self.ping_amount().get(),
            "Invalid ping amount. Please send exactly the required amount"
        );

        let caller = self.blockchain().get_caller();
        require!(
            !self.user_ping_timestamp(&caller).is_empty(),
            "User already has an active ping"
        );

        self.user_ping_timestamp(&caller)
            .set(self.blockchain().get_block_timestamp());
    }

    #[endpoint]
    fn pong(&self) {
        let caller = self.blockchain().get_caller();
        let ping_timestamp = self.user_ping_timestamp(&caller).get();
        
        require!(!ping_timestamp.is_empty(), "No active ping found");

        let current_timestamp = self.blockchain().get_block_timestamp();
        let time_passed = current_timestamp - ping_timestamp;
        
        require!(
            time_passed >= self.duration_in_seconds().get(),
            "Too early to pong"
        );

        // Clear storage
        self.user_ping_timestamp(&caller).clear();

        // Send back tokens
        let amount = self.ping_amount().get();
        self.send().direct_egld(&caller, &amount);
    }

    // Storage

    #[view(getPingAmount)]
    #[storage_mapper("pingAmount")]
    fn ping_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getDurationInSeconds)]
    #[storage_mapper("durationInSeconds")]
    fn duration_in_seconds(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("userPingTimestamp")]
    fn user_ping_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;
} 