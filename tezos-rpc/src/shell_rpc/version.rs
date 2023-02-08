use crate::{http::Http, models::version::VersionInfo};

use {crate::client::TezosRpcContext, crate::error::Error};

fn path() -> &'static str {
    "/version"
}

/// A builder to construct the properties of a request to get the node version.
#[derive(Clone, Copy)]
pub struct RpcRequestBuilder<'a, HttpClient: Http> {
    ctx: &'a TezosRpcContext<HttpClient>,
}

impl<'a, HttpClient: Http> RpcRequestBuilder<'a, HttpClient> {
    pub fn new(ctx: &'a TezosRpcContext<HttpClient>) -> Self {
        RpcRequestBuilder {
            ctx,
        }
    }

    pub async fn send(&self) -> Result<VersionInfo, Error> {
        self.ctx.http_client().get(self::path()).await
    }
}

/// Get node version.
///
/// [`GET /version`](https://tezos.gitlab.io/shell/rpc.html#get-version)
pub fn get<HttpClient: Http>(ctx: &TezosRpcContext<HttpClient>) -> RpcRequestBuilder<HttpClient> {
    RpcRequestBuilder::new(ctx)
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use {crate::client::TezosRpc, crate::models::version::VersionInfo, crate::error::Error, httpmock::prelude::*};

    #[tokio::test]
    async fn test_get_version() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let version_info = serde_json::json!({
            "version": {
                "major": 15,
                "minor": 1,
                "additional_info": "release"
            },
            "network_version": {
                "chain_name": "TEZOS_MAINNET",
                "distributed_db_version": 2,
                "p2p_version": 1
            },
            "commit_info": {
                "commit_hash": "763259c5131a5cc8054151596f0f59ffb505f0fc",
                "commit_date": "2022-12-01 10:20:58 +0000"
            }
        });

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path());
            then.status(200)
                .header("content-type", "application/json")
                .json_body(version_info.clone());
        });

        let client = TezosRpc::new(rpc_url);
        let actual = client.get_version().send().await?;
        let expected: VersionInfo = serde_json::from_value(version_info)?;
        assert_eq!(expected, actual);

        Ok(())
    }
}
