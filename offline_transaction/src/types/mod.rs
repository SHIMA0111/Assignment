use serde::{Deserialize, Serialize};

pub mod bitcoin;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Network {
    pub(crate) network: String,
}
