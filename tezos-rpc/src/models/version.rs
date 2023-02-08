use {
    serde::{Deserialize, Serialize},
    derive_more::Display,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalInfo {
    Release,
    Dev,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub additional_info: AdditionalInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NetworkVersion {
    pub chain_name: String,
    pub distributed_db_version: u16,
    pub p2p_version: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommitInfo {
    pub commit_hash: String,
    pub commit_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VersionInfo {
    pub version: Version,
    pub network_version: NetworkVersion,
    pub commit_info: CommitInfo,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use serde_json::json;

    #[test]
    fn test_serde_serialize() -> Result<(), crate::error::Error> {
        let info = VersionInfo {
            version: Version {
                major: 0,
                minor: 0,
                additional_info: AdditionalInfo::Dev,
            },
            network_version: NetworkVersion {
                chain_name: "TEZOS-MONDAYNET-2023-02-06T00:00:00.000Z".into(),
                distributed_db_version: 2,
                p2p_version: 1,
            },
            commit_info: CommitInfo {
                commit_hash: "0b861fd0".into(),
                commit_date: "2023-02-03 20:04:57 +0000".into(),
            }
        };
        let json = serde_json::to_value(&info)?;

        let expected = json!({
            "version": {
                "major": 0,
                "minor": 0,
                "additional_info": "dev"
            },
            "network_version": {
                "chain_name": "TEZOS-MONDAYNET-2023-02-06T00:00:00.000Z",
                "distributed_db_version": 2,
                "p2p_version": 1
            },
            "commit_info": {
                "commit_hash": "0b861fd0",
                "commit_date": "2023-02-03 20:04:57 +0000"
            }
        });

        assert_eq!(json, expected);

        Ok(())
    }

    #[test]
    fn test_serde_deserialize() -> Result<(), crate::error::Error> {
        let json = json!({
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

        let info: VersionInfo = serde_json::from_value(json)?;

        assert_eq!(info.version.major, 15);
        assert_eq!(info.version.minor, 1);
        assert_eq!(info.version.additional_info, AdditionalInfo::Release);
        assert_eq!(info.network_version.chain_name, "TEZOS_MAINNET");
        assert_eq!(info.network_version.distributed_db_version, 2);
        assert_eq!(info.network_version.p2p_version, 1);
        assert_eq!(info.commit_info.commit_hash, "763259c5131a5cc8054151596f0f59ffb505f0fc");
        assert_eq!(info.commit_info.commit_date, "2022-12-01 10:20:58 +0000");

        Ok(())
    }
}
