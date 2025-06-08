#![no_std]

use multiversx_sc::{derive_imports::*, imports::*};
pub mod crowdfunding_proxy;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[multiversx_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, target: BigUint) {
        require!(target > 0, "Target must be more than 0");
        self.target().set(target.clone());
    }

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        let payment = self.call_value().egld();
        require!(
            self.status() == Status::FundingPeriod,
            "cannot fund after realization"
        );

        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += &*payment);
    }

    #[view]
    fn status(&self) -> Status {
        if self.is_realized().get() {
            if self.get_current_funds() >= self.target().get() {
                Status::Successful
            } else {
                Status::Failed
            }
        } else {
            Status::FundingPeriod
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    #[endpoint]
    fn realize(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can realize the funding"
        );
        require!(!self.is_realized().get(), "funding already realized");

        self.is_realized().set(true);
    }

    #[endpoint]
    fn claim(&self) {
        require!(self.is_realized().get(), "funding not realized yet");
        
        match self.status() {
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                require!(sc_balance > 0, "no funds to claim");
                
                self.send().direct_egld(&caller, &sc_balance);
                self.emit_claim_event(&caller, &sc_balance, Status::Successful);
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);
                    self.emit_claim_event(&caller, &deposit, Status::Failed);
                }
            },
            _ => sc_panic!("invalid status for claim"),
        }
    }

    #[event("claim")]
    fn emit_claim_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] amount: &BigUint,
        #[indexed] status: Status,
    );

    // storage

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("is_realized")]
    fn is_realized(&self) -> SingleValueMapper<bool>;
}