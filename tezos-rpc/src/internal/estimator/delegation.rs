use tezos_core::types::number::Nat;
use crate::models::{
    error::RpcError,
    operation::{
        operation_contents_and_result::delegation::DelegationMetadata,
        operation_result::{
            operations::{delegation::DelegationOperationResult, InternalOperationResult},
            OperationResultStatus,
        },
    },
};

use super::{RpcMetadata, RpcOperationResult};

impl RpcOperationResult for DelegationOperationResult {
    fn status(&self) -> OperationResultStatus {
        self.status
    }

    fn number_of_originated_contracts(&self) -> usize {
        0
    }

    fn consumed_gas(&self) -> Nat {
        self.consumed_gas
            .as_ref()
            .map_or(0u32.into(), |consumed_gas| {
                consumed_gas.parse().unwrap_or(0u8.into())
            })
    }

    fn consumed_milligas(&self) -> Nat {
        self.consumed_milligas
            .as_ref()
            .map_or(0u32.into(), |consumed_gas| {
                consumed_gas.parse().unwrap_or(0u8.into())
            })
    }

    fn paid_storage_size_diff(&self) -> Option<Nat> {
        None
    }

    fn allocated_destination_contract(&self) -> Option<bool> {
        None
    }

    fn errors(&self) -> Option<&Vec<RpcError>> {
        self.errors.as_ref()
    }
}

impl RpcMetadata<DelegationOperationResult> for DelegationMetadata {
    fn operation_result(&self) -> &DelegationOperationResult {
        &self.operation_result
    }

    fn internal_operation_results(&self) -> Option<&Vec<InternalOperationResult>> {
        None
    }
}
