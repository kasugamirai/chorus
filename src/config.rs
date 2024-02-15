use crate::error::Error;
use crate::types::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendlyConfig {
    pub data_directory: String,
    pub ip_address: String,
    pub port: u16,
    pub use_tls: bool,
    pub certchain_pem_path: String,
    pub key_pem_path: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public_key_hex: Option<String>,
    pub user_hex_keys: Vec<String>,
    pub verify_events: bool,
}

impl Default for FriendlyConfig {
    fn default() -> FriendlyConfig {
        FriendlyConfig {
            data_directory: "/tmp".to_string(),
            ip_address: "127.0.0.1".to_string(),
            port: 80,
            use_tls: false,
            certchain_pem_path: "./tls/fullchain.pem".to_string(),
            key_pem_path: "./tls/privkey.pem".to_string(),
            name: None,
            description: None,
            public_key_hex: None,
            user_hex_keys: vec![],
            verify_events: true,
        }
    }
}

impl FriendlyConfig {
    pub fn into_config(self) -> Result<Config, Error> {
        let FriendlyConfig {
            data_directory,
            ip_address,
            port,
            use_tls,
            certchain_pem_path,
            key_pem_path,
            name,
            description,
            public_key_hex,
            user_hex_keys,
            verify_events,
        } = self;

        let mut public_key: Option<Pubkey> = None;
        if let Some(pkh) = public_key_hex {
            public_key = Some(Pubkey::read_hex(pkh.as_bytes())?);
        };

        let mut user_keys: Vec<Pubkey> = Vec::with_capacity(user_hex_keys.len());
        for pkh in user_hex_keys.iter() {
            user_keys.push(Pubkey::read_hex(pkh.as_bytes())?);
        }

        Ok(Config {
            data_directory,
            ip_address,
            port,
            use_tls,
            certchain_pem_path,
            key_pem_path,
            name,
            description,
            public_key,
            user_keys,
            user_hex_keys,
            verify_events,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub data_directory: String,
    pub ip_address: String,
    pub port: u16,
    pub use_tls: bool,
    pub certchain_pem_path: String,
    pub key_pem_path: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public_key: Option<Pubkey>,
    pub user_keys: Vec<Pubkey>,
    pub user_hex_keys: Vec<String>,
    pub verify_events: bool,
}
