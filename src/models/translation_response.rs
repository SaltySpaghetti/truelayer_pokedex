#![allow(warnings)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TranslationResponse {
    pub success: Success,
    pub contents: Contents,
}

#[derive(Serialize, Deserialize)]
pub struct Contents {
    pub translated: String,
    pub text: String,
    pub translation: String,
}

#[derive(Serialize, Deserialize)]
pub struct Success {
    pub total: i64,
}
