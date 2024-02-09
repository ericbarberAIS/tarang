// lib.rs

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
// #[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub seed: u64,
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}
