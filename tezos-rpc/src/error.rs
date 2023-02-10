use {
    crate::models::error::RpcErrors,
    derive_more::{Display, Error, From},
};

#[derive(Debug, From, Display, Error)]
pub enum Error {
    Core {
        source: tezos_core::Error,
    },
    Operation {
        source: tezos_operation::Error,
    },
    #[cfg(feature = "http")]
    HttpError {
        source: reqwest::Error,
    },
    #[cfg(any(test, feature = "serde_json"))]
    ParsingError {
        source: serde_json::Error,
    },
    ParseIntError {
        source: std::num::ParseIntError,
    },
    RpcErrorPlain {
        description: String,
    },
    RpcErrors(#[error(not(source))] RpcErrors),
    InvalidConversion,
    OperationNotSupported,
}

pub type Result<T> = std::result::Result<T, Error>;
