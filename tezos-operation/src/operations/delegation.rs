use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone)]
pub struct Delegation {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    delegate: Option<ImplicitAddress>,
}

impl Delegation {
    pub fn delegate(&self) -> Option<&ImplicitAddress> {
        self.delegate.as_ref()
    }

    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        delegate: Option<ImplicitAddress>,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            delegate,
        }
    }
}

impl TraitOperationContent for Delegation {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::Delegation as u8]
    }
}

impl TraitOperationManagerContent for Delegation {
    fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    fn fee(&self) -> Mutez {
        self.fee
    }

    fn counter(&self) -> &Nat {
        &self.counter
    }

    fn gas_limit(&self) -> &Nat {
        &self.gas_limit
    }

    fn storage_limit(&self) -> &Nat {
        &self.storage_limit
    }
}
