use tezos_core::types::{
    encoded::{ImplicitAddress, PublicKey},
    mutez::Mutez,
    number::Nat,
};

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone)]
pub struct Reveal {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    public_key: PublicKey,
}

impl Reveal {
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        public_key: PublicKey,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            public_key,
        }
    }
}

impl TraitOperationContent for Reveal {
    fn tag() -> &'static [u8] {
        &[OperationContentTag::Reveal as u8]
    }
}

impl TraitOperationManagerContent for Reveal {
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
