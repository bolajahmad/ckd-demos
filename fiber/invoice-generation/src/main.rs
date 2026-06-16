use ckb_sdk::rpc::ckb_indexer::SearchKey;
use ckb_sdk::CkbRpcClient;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const NODES_DIR: &str = "nodes";
const OUT_FILE: &str = "out.txt";
const CKB_RPC_URL: &str = "https://testnet.ckbapp.dev/";
const CHANNEL_FUNDING_SHANNONS: &str = "10000000000";
const INVOICE_AMOUNT_SHANNONS: &str = "1000";
const INVOICE_DESCRIPTION: &str = "invoice-generated-by-bob";
const INVOICE_EXPIRY_SECONDS: u64 = 3600;

#[derive(Debug, Clone, Deserialize)]
struct NodeConfigFile {
    fiber: FiberConfig,
    rpc: RpcConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct FiberConfig {
    listening_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcConfig {
    listening_addr: String,
}

#[derive(Debug, Clone)]
struct NodeConfig {
    name: String,
    workdir: PathBuf,
    rpc_url: String,
    p2p_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FundingLockScript {
    code_hash: String,
    hash_type: String,
    args: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct NodeInfo {
    version: String,
    commit_hash: String,
    pubkey: String,
    features: Vec<String>,
    node_name: Option<String>,
    addresses: Vec<String>,
    chain_hash: String,
    open_channel_auto_accept_min_ckb_funding_amount: String,
    auto_accept_channel_ckb_funding_amount: String,
    default_funding_lock_script: FundingLockScript,
    tlc_expiry_delta: String,
    tlc_min_value: String,
    tlc_fee_proportional_millionths: String,
    channel_count: String,
    pending_channel_count: String,
    peers_count: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ChannelListResponse {
    channels: Vec<ChannelSummary>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ChannelSummary {
    channel_id: String,
    channel_outpoint: String,
    created_at: String,
    enabled: bool,
    failure_detail: Option<Value>,
    funding_udt_type_script: Option<Value>,
    is_acceptor: bool,
    is_one_way: bool,
    is_public: bool,
    latest_commitment_transaction_hash: String,
    local_balance: String,
    offered_tlc_balance: String,
    pending_tlcs: Vec<Value>,
    pubkey: String,
    received_tlc_balance: String,
    remote_balance: String,
    shutdown_transaction_hash: Option<String>,
    state: ChannelState,
    tlc_expiry_delta: String,
    tlc_fee_proportional_millionths: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ChannelState {
    state_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Default)]
struct PeerConnectResponse {}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Default)]
struct OpenChannelResponse {
    #[serde(default)]
    temporary_channel_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    channel_outpoint: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct InvoiceResponse {
    invoice: InvoiceEnvelope,
    invoice_address: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ParsedInvoiceResponse {
    invoice: InvoiceEnvelope,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct InvoiceEnvelope {
    amount: String,
    currency: String,
    data: InvoiceData,
    signature: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct InvoiceData {
    attrs: Vec<InvoiceAttribute>,
    payment_hash: String,
    timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
struct InvoiceAttribute {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    final_htlc_minimum_expiry_delta: Option<String>,
    #[serde(default)]
    payee_public_key: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct PaymentResponse {
    created_at: String,
    custom_records: Option<Value>,
    failed_error: Option<String>,
    fee: String,
    last_updated_at: String,
    payment_hash: String,
    status: PaymentStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum PaymentStatus {
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

#[derive(Debug, Clone)]
struct NodeReport {
    name: String,
    rpc_url: String,
    p2p_addr: String,
    pubkey: String,
    capacity_shannons: u64,
}

#[derive(Debug, Clone)]
struct ChannelReport {
    status: String,
    channel_id: String,
    channel_outpoint: String,
    local_balance: String,
    remote_balance: String,
    state: String,
}

#[derive(Debug, Clone)]
struct InvoiceReport {
    amount_shannons: String,
    description: String,
    recipient_name: String,
    recipient_pubkey: String,
    expiry_seconds: u64,
    invoice_address: String,
    payment_hash: String,
    final_htlc_minimum_expiry_delta: String,
}

#[derive(Debug, Clone)]
struct PaymentReport {
    payment_hash: String,
    status: String,
    fee: String,
    created_at: String,
    last_updated_at: String,
}

#[derive(Debug, Clone)]
struct RunReport {
    nodes: Vec<NodeReport>,
    channel: ChannelReport,
    invoice: InvoiceReport,
    payment: PaymentReport,
    steps: Vec<StepReport>,
}

#[derive(Debug, Clone, Deserialize)]
struct StepReport {
    step: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

struct FnnClient {
    name: String,
    workdir: PathBuf,
    rpc_url: String,
    cli_path: PathBuf,
}

impl FnnClient {
    fn new(name: String, workdir: PathBuf, rpc_url: String) -> Self {
        let cli_path = workdir.join("fnn-cli");
        Self {
            name,
            workdir,
            rpc_url,
            cli_path,
        }
    }

    fn run_raw(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        let mut command = Command::new(&self.cli_path);
        command.current_dir(&self.workdir);
        command.args(["-u", self.rpc_url.as_str(), "--raw-data", "-o", "json"]);
        command.args(args);

        let output = command.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!("{} failed: {stderr}; stdout: {stdout}", self.name).into());
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn run_json<T>(&self, args: &[String]) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let stdout = self.run_raw(args)?;
        Ok(serde_json::from_str(&stdout)?)
    }

    fn run_json_or_default<T>(&self, args: &[String]) -> Result<T, Box<dyn Error>>
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

trait NodeApi {
    fn node_info(&self) -> Result<NodeInfo, Box<dyn Error>>;
    fn capacity_shannons(&self, info: &NodeInfo) -> Result<u64, Box<dyn Error>>;
}

trait PeerApi {
    fn connect_peer_by_pubkey(&self, pubkey: &str) -> Result<PeerConnectResponse, Box<dyn Error>>;
}

trait ChannelApi {
    fn list_channels(
        &self,
        pubkey: Option<&str>,
        include_closed: bool,
    ) -> Result<ChannelListResponse, Box<dyn Error>>;

    fn open_channel(
        &self,
        peer_pubkey: &str,
        funding_amount_shannons: &str,
    ) -> Result<OpenChannelResponse, Box<dyn Error>>;

    fn accept_channel(
        &self,
        temporary_channel_id: &str,
        funding_amount_shannons: &str,
    ) -> Result<OpenChannelResponse, Box<dyn Error>>;
}

trait InvoiceApi {
    fn new_invoice(
        &self,
        amount_shannons: &str,
        description: &str,
        expiry_seconds: u64,
    ) -> Result<InvoiceResponse, Box<dyn Error>>;

    fn parse_invoice(&self, invoice_address: &str) -> Result<ParsedInvoiceResponse, Box<dyn Error>>;
}

trait PaymentApi {
    fn send_payment(&self, invoice_address: &str) -> Result<PaymentResponse, Box<dyn Error>>;
    fn get_payment(&self, payment_hash: &str) -> Result<PaymentResponse, Box<dyn Error>>;
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

impl PeerApi for FnnClient {
    fn connect_peer_by_pubkey(&self, pubkey: &str) -> Result<PeerConnectResponse, Box<dyn Error>> {
        self.run_json_or_default(&[
            "peer".to_string(),
            "connect_peer".to_string(),
            "--pubkey".to_string(),
            pubkey.to_string(),
        ])
    }
}

impl ChannelApi for FnnClient {
    fn list_channels(
        &self,
        pubkey: Option<&str>,
        include_closed: bool,
    ) -> Result<ChannelListResponse, Box<dyn Error>> {
        let mut args = vec!["channel".to_string(), "list_channels".to_string()];
        if let Some(pubkey) = pubkey {
            args.push("--pubkey".to_string());
            args.push(pubkey.to_string());
        }
        if include_closed {
            args.push("--include-closed".to_string());
            args.push("true".to_string());
        }
        self.run_json(&args)
    }

    fn open_channel(
        &self,
        peer_pubkey: &str,
        funding_amount_shannons: &str,
    ) -> Result<OpenChannelResponse, Box<dyn Error>> {
        self.run_json_or_default(&[
            "channel".to_string(),
            "open_channel".to_string(),
            "--pubkey".to_string(),
            peer_pubkey.to_string(),
            "--funding-amount".to_string(),
            funding_amount_shannons.to_string(),
        ])
    }

    fn accept_channel(
        &self,
        temporary_channel_id: &str,
        funding_amount_shannons: &str,
    ) -> Result<OpenChannelResponse, Box<dyn Error>> {
        self.run_json_or_default(&[
            "channel".to_string(),
            "accept_channel".to_string(),
            "--temporary-channel-id".to_string(),
            temporary_channel_id.to_string(),
            "--funding-amount".to_string(),
            funding_amount_shannons.to_string(),
        ])
    }
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

    fn parse_invoice(&self, invoice_address: &str) -> Result<ParsedInvoiceResponse, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let discovered = discover_nodes(Path::new(NODES_DIR))?;
    let clients: Vec<(NodeConfig, FnnClient)> = discovered
        .into_iter()
        .map(|node| {
            let client = FnnClient::new(node.name.clone(), node.workdir.clone(), node.rpc_url.clone());
            (node, client)
        })
        .collect();

    let mut steps = Vec::<StepReport>::new();
    steps.push(StepReport {
        step: "discover_nodes".to_string(),
        status: "ok".to_string(),
        node: None,
        details: Some(serde_json::json!({
            "node_count": clients.len(),
            "names": clients.iter().map(|(node, _)| node.name.clone()).collect::<Vec<_>>()
        })),
    });

    let mut reports = Vec::<NodeReport>::new();
    let mut alice_client: Option<FnnClient> = None;
    let mut bob_client: Option<FnnClient> = None;

    for (node, client) in &clients {
        let info = client.node_info()?;
        let capacity_shannons = client.capacity_shannons(&info)?;

        steps.push(StepReport {
            step: "node_info".to_string(),
            status: "ok".to_string(),
            node: Some(node.name.clone()),
            details: Some(serde_json::json!({
                "name": node.name,
                "rpc_url": node.rpc_url,
                "pubkey": info.pubkey,
                "capacity_shannons": capacity_shannons,
            })),
        });

        if node.name.eq_ignore_ascii_case("alice") {
            alice_client = Some(FnnClient::new(
                node.name.clone(),
                node.workdir.clone(),
                node.rpc_url.clone(),
            ));
        } else if node.name.eq_ignore_ascii_case("bob") {
            bob_client = Some(FnnClient::new(
                node.name.clone(),
                node.workdir.clone(),
                node.rpc_url.clone(),
            ));
        }

        reports.push(NodeReport {
            name: node.name.clone(),
            rpc_url: node.rpc_url.clone(),
            p2p_addr: node.p2p_addr.clone(),
            pubkey: info.pubkey,
            capacity_shannons,
        });
    }

    let alice_client = alice_client.ok_or("alice node is missing")?;
    let bob_client = bob_client.ok_or("bob node is missing")?;

    let alice_info = alice_client.node_info()?;
    let bob_info = bob_client.node_info()?;

    alice_client.connect_peer_by_pubkey(&bob_info.pubkey)?;
    bob_client.connect_peer_by_pubkey(&alice_info.pubkey)?;
    steps.push(StepReport {
        step: "connect_peers".to_string(),
        status: "ok".to_string(),
        node: None,
        details: Some(serde_json::json!({
            "alice_peer": bob_info.pubkey,
            "bob_peer": alice_info.pubkey,
        })),
    });

    let existing_channel = find_ready_channel(&alice_client, &bob_info.pubkey)?;
    let channel_report = if let Some(channel) = existing_channel {
        steps.push(StepReport {
            step: "channel_open".to_string(),
            status: "already_ready".to_string(),
            node: Some("alice".to_string()),
            details: Some(serde_json::json!({
                "channel_id": channel.channel_id,
                "channel_outpoint": channel.channel_outpoint,
                "local_balance": channel.local_balance,
                "remote_balance": channel.remote_balance,
                "state": channel.state.state_name,
            })),
        });

        ChannelReport {
            status: "already_ready".to_string(),
            channel_id: channel.channel_id,
            channel_outpoint: channel.channel_outpoint,
            local_balance: channel.local_balance,
            remote_balance: channel.remote_balance,
            state: channel.state.state_name,
        }
    } else {
        let open_response = alice_client.open_channel(&bob_info.pubkey, CHANNEL_FUNDING_SHANNONS)?;
        steps.push(StepReport {
            step: "channel_open".to_string(),
            status: "requested".to_string(),
            node: Some("alice".to_string()),
            details: Some(serde_json::json!({
                "temporary_channel_id": open_response.temporary_channel_id,
                "channel_id": open_response.channel_id,
                "channel_outpoint": open_response.channel_outpoint,
            })),
        });

        if let Some(temp_id) = open_response.temporary_channel_id.as_deref() {
            let accept_response = bob_client.accept_channel(temp_id, CHANNEL_FUNDING_SHANNONS)?;
            steps.push(StepReport {
                step: "channel_accept".to_string(),
                status: "requested".to_string(),
                node: Some("bob".to_string()),
                details: Some(serde_json::json!({
                    "temporary_channel_id": temp_id,
                    "channel_id": accept_response.channel_id,
                    "channel_outpoint": accept_response.channel_outpoint,
                })),
            });
        }

        let ready_channel = wait_for_ready_channel(&alice_client, &bob_info.pubkey)?;
        let Some(channel) = ready_channel else {
            return Err("channel did not become ready".into());
        };

        ChannelReport {
            status: "opened".to_string(),
            channel_id: channel.channel_id,
            channel_outpoint: channel.channel_outpoint,
            local_balance: channel.local_balance,
            remote_balance: channel.remote_balance,
            state: channel.state.state_name,
        }
    };

    let invoice_response = bob_client.new_invoice(
        INVOICE_AMOUNT_SHANNONS,
        INVOICE_DESCRIPTION,
        INVOICE_EXPIRY_SECONDS,
    )?;
    let parsed_invoice = bob_client.parse_invoice(&invoice_response.invoice_address)?;

    let parsed_invoice_envelope = &parsed_invoice.invoice;
    let invoice_attrs = &parsed_invoice_envelope.data.attrs;
    let description = invoice_attrs
        .iter()
        .find_map(|attr| attr.description.clone())
        .unwrap_or_else(|| INVOICE_DESCRIPTION.to_string());
    let recipient_pubkey = invoice_attrs
        .iter()
        .find_map(|attr| attr.payee_public_key.clone())
        .unwrap_or_else(|| bob_info.pubkey.clone());
    let final_htlc_minimum_expiry_delta = invoice_attrs
        .iter()
        .find_map(|attr| attr.final_htlc_minimum_expiry_delta.clone())
        .unwrap_or_default();

    let invoice_report = InvoiceReport {
        amount_shannons: parsed_invoice_envelope.amount.clone(),
        description,
        recipient_name: "bob".to_string(),
        recipient_pubkey,
        expiry_seconds: INVOICE_EXPIRY_SECONDS,
        invoice_address: invoice_response.invoice_address.clone(),
        payment_hash: parsed_invoice_envelope.data.payment_hash.clone(),
        final_htlc_minimum_expiry_delta,
    };

    steps.push(StepReport {
        step: "invoice_created".to_string(),
        status: "ok".to_string(),
        node: Some("bob".to_string()),
        details: Some(serde_json::json!({
            "amount_shannons": invoice_report.amount_shannons,
            "description": invoice_report.description,
            "recipient_pubkey": invoice_report.recipient_pubkey,
            "invoice_address": invoice_report.invoice_address,
            "payment_hash": invoice_report.payment_hash,
            "expiry_seconds": invoice_report.expiry_seconds,
        })),
    });

    let send_payment_response = alice_client.send_payment(&invoice_report.invoice_address)?;
    let confirmed_payment = wait_for_payment_success(&alice_client, &send_payment_response.payment_hash)?;

    let payment_report = PaymentReport {
        payment_hash: confirmed_payment.payment_hash.clone(),
        status: format_status(&confirmed_payment.status),
        fee: confirmed_payment.fee.clone(),
        created_at: confirmed_payment.created_at.clone(),
        last_updated_at: confirmed_payment.last_updated_at.clone(),
    };

    steps.push(StepReport {
        step: "payment_sent".to_string(),
        status: "ok".to_string(),
        node: Some("alice".to_string()),
        details: Some(serde_json::json!({
            "payment_hash": payment_report.payment_hash,
            "status": payment_report.status,
            "fee": payment_report.fee,
        })),
    });

    let report = RunReport {
        nodes: reports,
        channel: channel_report,
        invoice: invoice_report,
        payment: payment_report,
        steps,
    };

    fs::write(OUT_FILE, serde_json::to_string_pretty(&report_to_value(&report))?)?;
    println!("Flow completed. Wrote concise run report to {OUT_FILE}");

    Ok(())
}

fn discover_nodes(nodes_dir: &Path) -> Result<Vec<NodeConfig>, Box<dyn Error>> {
    let mut nodes = Vec::new();

    for entry in fs::read_dir(nodes_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let workdir = entry.path().canonicalize()?;
        let config_path = workdir.join("config.yml");
        if !config_path.exists() {
            continue;
        }

        let cfg: NodeConfigFile = serde_yaml::from_str(&fs::read_to_string(config_path)?)?;
        nodes.push(NodeConfig {
            name,
            workdir,
            rpc_url: format!("http://{}", cfg.rpc.listening_addr),
            p2p_addr: normalize_p2p_addr(&cfg.fiber.listening_addr)?,
        });
    }

    nodes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(nodes)
}

fn normalize_p2p_addr(addr: &str) -> Result<String, Box<dyn Error>> {
    let parts: Vec<&str> = addr.split('/').collect();
    if parts.len() < 5 || parts[1] != "ip4" || parts[3] != "tcp" {
        return Err(format!("unsupported listening_addr format: {addr}").into());
    }

    let host = if parts[2] == "0.0.0.0" {
        "127.0.0.1"
    } else {
        parts[2]
    };
    Ok(format!("/ip4/{host}/tcp/{}", parts[4]))
}

fn find_ready_channel(
    client: &FnnClient,
    peer_pubkey: &str,
) -> Result<Option<ChannelSummary>, Box<dyn Error>> {
    let channels = client.list_channels(Some(peer_pubkey), true)?;
    Ok(channels
        .channels
        .into_iter()
        .find(|channel| channel.state.state_name == "ChannelReady"))
}

fn wait_for_ready_channel(
    client: &FnnClient,
    peer_pubkey: &str,
) -> Result<Option<ChannelSummary>, Box<dyn Error>> {
    for _ in 0..20 {
        if let Some(channel) = find_ready_channel(client, peer_pubkey)? {
            return Ok(Some(channel));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(None)
}

fn wait_for_payment_success(client: &FnnClient, payment_hash: &str) -> Result<PaymentResponse, Box<dyn Error>> {
    let mut last_response = client.get_payment(payment_hash)?;

    for _ in 0..20 {
        match last_response.status {
            PaymentStatus::Success => return Ok(last_response),
            PaymentStatus::Failed | PaymentStatus::Cancelled | PaymentStatus::Canceled | PaymentStatus::Expired => {
                return Err(format!(
                    "payment {} ended in status {}",
                    payment_hash,
                    format_status(&last_response.status)
                )
                .into())
            }
            _ => {
                std::thread::sleep(std::time::Duration::from_secs(1));
                last_response = client.get_payment(payment_hash)?;
            }
        }
    }

    Ok(last_response)
}

fn parse_hex_u64(value: &str) -> Result<u64, Box<dyn Error>> {
    let clean = value.trim_start_matches("0x");
    Ok(u64::from_str_radix(clean, 16)?)
}

fn format_status(status: &PaymentStatus) -> String {
    match status {
        PaymentStatus::Created => "Created".to_string(),
        PaymentStatus::Success => "Success".to_string(),
        PaymentStatus::Failed => "Failed".to_string(),
        PaymentStatus::Pending => "Pending".to_string(),
        PaymentStatus::Expired => "Expired".to_string(),
        PaymentStatus::Cancelled => "Cancelled".to_string(),
        PaymentStatus::Canceled => "Canceled".to_string(),
        PaymentStatus::Inflight => "Inflight".to_string(),
        PaymentStatus::Unknown => "Unknown".to_string(),
    }
}

fn report_to_value(report: &RunReport) -> Value {
    serde_json::json!({
        "nodes": report.nodes.iter().map(|node| serde_json::json!({
            "name": node.name,
            "rpc_url": node.rpc_url,
            "p2p_addr": node.p2p_addr,
            "pubkey": node.pubkey,
            "capacity_shannons": node.capacity_shannons,
        })).collect::<Vec<_>>(),
        "channel": {
            "status": report.channel.status,
            "channel_id": report.channel.channel_id,
            "channel_outpoint": report.channel.channel_outpoint,
            "local_balance": report.channel.local_balance,
            "remote_balance": report.channel.remote_balance,
            "state": report.channel.state,
        },
        "invoice": {
            "amount_shannons": report.invoice.amount_shannons,
            "description": report.invoice.description,
            "recipient": {
                "name": report.invoice.recipient_name,
                "pubkey": report.invoice.recipient_pubkey,
            },
            "expiry_seconds": report.invoice.expiry_seconds,
            "invoice_address": report.invoice.invoice_address,
            "payment_hash": report.invoice.payment_hash,
            "final_htlc_minimum_expiry_delta": report.invoice.final_htlc_minimum_expiry_delta,
        },
        "payment": {
            "payment_hash": report.payment.payment_hash,
            "status": report.payment.status,
            "fee": report.payment.fee,
            "created_at": report.payment.created_at,
            "last_updated_at": report.payment.last_updated_at,
        },
        "steps": report.steps.iter().map(|step| serde_json::json!({
            "step": step.step,
            "status": step.status,
            "node": step.node,
            "details": step.details,
        })).collect::<Vec<_>>(),
    })
}
