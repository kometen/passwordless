use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct PasswordOptions {
    pub count: Option<usize>,
    pub length: Option<usize>,
    pub numbers: Option<bool>,
    pub lowercase_letters: Option<bool>,
    pub uppercase_letters: Option<bool>,
    pub symbols: Option<bool>,
    pub spaces: Option<bool>,
    pub exclude_similar_characters: Option<bool>,
    pub strict: Option<bool>,
}

impl PasswordOptions {
    pub fn new(count: usize, length: usize) -> PasswordOptions {
        PasswordOptions {
            count: Some(count),
            length: Some(length),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Pwd {
    pub password: String,
    pub score: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    #[serde(rename = "displayname")]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVerifyRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVerifyResponse {
    pub success: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub timestamp: String,
    pub rpid: String,
    pub origin: String,
    pub device: String,
    pub country: String,
    pub nickname: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    pub token_id: String,
    #[serde(rename = "type")]
    pub stype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCredentialRequest {
    #[serde(rename = "credentialId")]
    pub credential_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDescriptor {
    #[serde(rename = "type")]
    pub typec: String,
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub descriptor: CredentialDescriptor,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "userHandle")]
    pub user_handle: String,
    #[serde(rename = "signatureCounter")]
    pub signature_counter: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "aaGuid")]
    pub aa_guid: String,
    #[serde(rename = "lastUsedAt")]
    pub last_used_at: String,
    pub rpid: String,
    pub origin: String,
    pub country: String,
    pub device: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AliasRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub hashing: bool,
    pub aliases: Vec<String>,
}
