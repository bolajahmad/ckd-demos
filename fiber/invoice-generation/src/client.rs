use std::{error::Error, path::PathBuf, process::Command};

use ckb_sdk::{CkbRpcClient, rpc::ckb_indexer::SearchKey};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::Value;

const CKB_RPC_URL: &str = "https://testnet.ckbapp.dev/";

pub struct FnnClient {
    pub name: String,
    pub workdir: PathBuf,
    pub rpc_url: String,
    pub cli_path: PathBuf,
}

impl FnnClient {
    pub fn new(name: String, workdir: PathBuf, rpc_url: String) -> Self {
        let cli_path = workdir.join("fnn-cli");
        Self {
            name,
            workdir,
            rpc_url,
            cli_path,
        }
    }

    pub fn run_raw(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        let mut command = Command::new(&self.cli_path);
        command.current_dir(&self.workdir);
        command.args(["--url", self.rpc_url.as_str(), "--raw-data", "-o", "json"]);
        command.args(args);

        let output = command.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!("{} failed: {stderr}; stdout: {stdout}", self.name).into());
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn run_json<T>(&self, args: &[String]) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let stdout = self.run_raw(args)?;
        Ok(serde_json::from_str(&stdout)?)
    }

    pub fn run_json_or_default<T>(&self, args: &[String]) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned + Default,
    {
        let stdout = self.run_raw(args)?;
        if stdout.is_empty() {
            return Ok(T::default());
        }
        Ok(serde_json::from_str(&stdout)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FundingLockScript {
    pub code_hash: String,
    pub hash_type: String,
    pub args: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct NodeInfo {
    pub version: String,
    pub pubkey: String,
    pub commit_hash: String,
    pub addresses: Vec<String>,
    pub chain_hash: String,
    pub open_channel_auto_accept_min_ckb_funding_amount: String,
    pub auto_accept_channel_ckb_funding_amount: String,
    pub default_funding_lock_script: FundingLockScript,
    pub tlc_expiry_delta: String,
    pub tlc_min_value: String,
    pub channel_count: String,
    pub pending_channel_count: String,
    pub peers_count: String,
}

pub trait NodeApi {
    fn node_info(&self) -> Result<NodeInfo, Box<dyn Error>>;
    fn capacity_shannons(&self, info: &NodeInfo) -> Result<u64, Box<dyn Error>>;
}

impl NodeApi for FnnClient {
    fn node_info(&self) -> Result<NodeInfo, Box<dyn Error>> {
        self.run_json(&["info".to_string(), "node_info".to_string()])
    }

    fn capacity_shannons(&self, info: &NodeInfo) -> Result<u64, Box<dyn Error>> {
        let ckb_client = CkbRpcClient::new(CKB_RPC_URL);
        let search_key: SearchKey = serde_json::from_value(serde_json::json!({
            "script": {
                "code_hash": info.default_funding_lock_script.code_hash,
                "hash_type": info.default_funding_lock_script.hash_type,
                "args": info.default_funding_lock_script.args,
            },
            "script_type": "lock",
            "script_search_mode": "exact",
        }))?;

        let result = ckb_client
            .get_cells_capacity(search_key)?
            .ok_or_else(|| format!("capacity lookup returned no result for node {}", self.name))?;

        parse_hex_u64(&result.capacity.to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PeerResponse {}

pub trait PeerApi {
    fn connect_peer_by_pubkey(&self, pubkey: &str) -> Result<PeerResponse, Box<dyn Error>>;
}

impl PeerApi for FnnClient {
    fn connect_peer_by_pubkey(&self, pubkey: &str) -> Result<PeerResponse, Box<dyn Error>> {
        self.run_json_or_default(&[
            "peer".to_string(),
            "connect_peer".to_string(),
            "--pubkey".to_string(),
            pubkey.to_string(),
        ])
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
    pub channel_id: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    is_acceptor: bool,
    #[serde(default)]
    is_one_way: bool,
    #[serde(default)]
    is_public: bool,
    #[serde(default)]
    latest_commitment_transaction_hash: String,
    #[serde(default)]
    local_balance: String,
    #[serde(default)]
    offered_tlc_balance: String,
    #[serde(default)]
    pubkey: String,
    #[serde(default)]
    received_tlc_balance: String,
    pub remote_balance: String,
    #[serde(deserialize_with = "deserialize_channel_state")]
    pub state: String,
    #[serde(default)]
    tlc_expiry_delta: String,
    #[serde(default)]
    tlc_fee_proportional_millionths: String,
}

pub trait ChannelApi {
    fn list_channels(
        &self,
        pubkey: Option<&str>,
        include_closed: bool,
    ) -> Result<Vec<Channel>, Box<dyn Error>>;

    fn open_channel(
        &self,
        peer_pubkey: &str,
        funding_amount_shannons: &str,
    ) -> Result<Option<String>, Box<dyn Error>>;

    fn accept_channel(
        &self,
        temporary_channel_id: &str,
        funding_amount_shannons: &str,
    ) -> Result<Option<String>, Box<dyn Error>>;
}

impl ChannelApi for FnnClient {
    fn list_channels(
        &self,
        pubkey: Option<&str>,
        include_closed: bool,
    ) -> Result<Vec<Channel>, Box<dyn Error>> {
        let mut args = vec!["channel".to_string(), "list_channels".to_string()];
        if let Some(pubkey) = pubkey {
            args.push("--pubkey".to_string());
            args.push(pubkey.to_string());
        }
        if include_closed {
            args.push("--include-closed".to_string());
            args.push("true".to_string());
        }
        let response: Value = self.run_json(&args)?;

        let channels_value = response.get("channels").cloned().unwrap_or(response);

        Ok(serde_json::from_value(channels_value)?)
    }

    fn open_channel(
        &self,
        peer_pubkey: &str,
        funding_amount_shannons: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let response: Value = self.run_json_or_default(&[
            "channel".to_string(),
            "open_channel".to_string(),
            "--pubkey".to_string(),
            peer_pubkey.to_string(),
            "--funding-amount".to_string(),
            funding_amount_shannons.to_string(),
        ])?;

        Ok(extract_channel_reference(&response))
    }

    fn accept_channel(
        &self,
        temporary_channel_id: &str,
        funding_amount_shannons: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let response: Value = self.run_json_or_default(&[
            "channel".to_string(),
            "accept_channel".to_string(),
            "--temporary-channel-id".to_string(),
            temporary_channel_id.to_string(),
            "--funding-amount".to_string(),
            funding_amount_shannons.to_string(),
        ])?;

        Ok(extract_channel_reference(&response))
    }
}

fn deserialize_channel_state<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(state) => Ok(state),
        Value::Object(map) => map
            .get("state_name")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .ok_or_else(|| serde::de::Error::custom("state object missing state_name")),
        _ => Err(serde::de::Error::custom("unsupported state format")),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceAttribute {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub final_htlc_minimum_expiry_delta: Option<String>,
    #[serde(default)]
    pub payee_public_key: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceData {
    pub attrs: Vec<InvoiceAttribute>,
    pub payment_hash: String,
    #[serde(default)]
    pub timestamp: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Invoice {
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    pub data: InvoiceData,
    #[serde(default)]
    pub signature: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceResponse {
    pub invoice: Invoice,
    #[serde(default)]
    pub invoice_address: String,
}

pub trait InvoiceApi {
    fn new_invoice(
        &self,
        amount_shannons: &str,
        description: &str,
        expiry_seconds: u64,
    ) -> Result<InvoiceResponse, Box<dyn Error>>;

    fn parse_invoice(&self, invoice_address: &str) -> Result<InvoiceResponse, Box<dyn Error>>;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    pub created_at: String,
    pub custom_records: Option<Value>,
    pub failed_error: Option<String>,
    pub fee: String,
    pub last_updated_at: String,
    pub payment_hash: String,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentStatus {
    Created,
    Success,
    Failed,
    Pending,
    Expired,
    Cancelled,
    Canceled,
    Inflight,
    #[serde(other)]
    Unknown,
}

pub trait PaymentApi {
    fn send_payment(&self, invoice_address: &str) -> Result<PaymentResponse, Box<dyn Error>>;
    fn get_payment(&self, payment_hash: &str) -> Result<PaymentResponse, Box<dyn Error>>;
}

impl InvoiceApi for FnnClient {
    fn new_invoice(
        &self,
        amount_shannons: &str,
        description: &str,
        expiry_seconds: u64,
    ) -> Result<InvoiceResponse, Box<dyn Error>> {
        self.run_json(&[
            "invoice".to_string(),
            "new_invoice".to_string(),
            "--amount".to_string(),
            amount_shannons.to_string(),
            "--currency".to_string(),
            "Fibt".to_string(),
            "--description".to_string(),
            description.to_string(),
            "--expiry".to_string(),
            expiry_seconds.to_string(),
        ])
    }

    fn parse_invoice(&self, invoice_address: &str) -> Result<InvoiceResponse, Box<dyn Error>> {
        self.run_json(&[
            "invoice".to_string(),
            "parse_invoice".to_string(),
            "--invoice".to_string(),
            invoice_address.to_string(),
        ])
    }
}

impl PaymentApi for FnnClient {
    fn send_payment(&self, invoice_address: &str) -> Result<PaymentResponse, Box<dyn Error>> {
        self.run_json(&[
            "payment".to_string(),
            "send_payment".to_string(),
            "--invoice".to_string(),
            invoice_address.to_string(),
        ])
    }

    fn get_payment(&self, payment_hash: &str) -> Result<PaymentResponse, Box<dyn Error>> {
        self.run_json(&[
            "payment".to_string(),
            "get_payment".to_string(),
            "--payment-hash".to_string(),
            payment_hash.to_string(),
        ])
    }
}

fn extract_channel_reference(value: &Value) -> Option<String> {
    value
        .get("temporary_channel_id")
        .and_then(Value::as_str)
        .or_else(|| value.get("channel_id").and_then(Value::as_str))
        .map(ToString::to_string)
}

fn parse_hex_u64(value: &str) -> Result<u64, Box<dyn Error>> {
    let clean = value.trim_start_matches("0x");
    Ok(u64::from_str_radix(clean, 16)?)
}
