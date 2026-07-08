use std::{fmt, str::FromStr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeKind {
    Unity,
}

impl fmt::Display for BridgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unity => f.write_str("unity"),
        }
    }
}

impl FromStr for BridgeKind {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "unity" => Ok(Self::Unity),
            _ => Err(anyhow!(
                "unsupported bridge type '{value}'; expected: unity"
            )),
        }
    }
}
