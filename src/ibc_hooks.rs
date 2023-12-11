use bech32_no_std::ToBase32;
use sha2::{Digest, Sha256};

#[derive(Clone, PartialEq, prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: String,

    #[prost(string, tag = "2")]
    pub amount: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: String,

    #[prost(string, tag = "2")]
    pub source_channel: String,

    #[prost(message, tag = "3")]
    pub token: Option<Coin>,

    #[prost(string, tag = "4")]
    pub sender: String,

    #[prost(string, tag = "5")]
    pub receiver: String,

    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,

    #[prost(string, tag = "8")]
    pub memo: String,

    #[prost(message, tag = "9")]
    pub fee: Option<IbcFee>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct IbcFee {
    #[prost(message, repeated, tag = "1")]
    pub recv_fee: Vec<Coin>,

    #[prost(message, repeated, tag = "2")]
    pub ack_fee: Vec<Coin>,

    #[prost(message, repeated, tag = "3")]
    pub timeout_fee: Vec<Coin>,
}

const SENDER_PREFIX: &str = "ibc-wasm-hook-intermediary";
pub fn derive_intermediate_sender(
    channel: &str,
    original_sender: &str,
    bech32_prefix: &str,
) -> Result<String, bech32_no_std::Error> {
    let sender_path = format!("{channel}/{original_sender}");

    let sender_hash_32 = prefixed_sha256(SENDER_PREFIX, &sender_path);

    bech32_no_std::encode(bech32_prefix, sender_hash_32.to_base32())
}

pub fn prefixed_sha256(prefix: &str, address: &str) -> [u8; 32] {
    let mut hasher = Sha256::default();

    hasher.update(prefix.as_bytes());
    let prefix_hash = hasher.finalize();

    let mut hasher = Sha256::default();

    hasher.update(prefix_hash);
    hasher.update(address.as_bytes());

    hasher.finalize().into()
}
