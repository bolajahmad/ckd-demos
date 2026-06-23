use std::{error::Error, fs, path::Path};

use crate::{
    NodeConfigFile, PaymentSummary,
    client::{Channel, ChannelApi, FnnClient, PaymentApi, PaymentResponse, PaymentStatus},
};

pub fn load_node(nodes_dir: &Path, node_name: &str) -> Result<FnnClient, Box<dyn Error>> {
    let workdir = nodes_dir.join(node_name);
    if !workdir.is_dir() {
        return Err(format!("{node_name} node is missing").into());
    }

    let config_path = workdir.join("config.yml");
    if !config_path.exists() {
        return Err(format!("{node_name} node is missing").into());
    }

    let cfg: NodeConfigFile = serde_yaml::from_str(&fs::read_to_string(config_path)?)?;
    Ok(FnnClient::new(
        node_name.to_string(),
        workdir.canonicalize()?,
        format!("http://{}", cfg.rpc.listening_addr),
    ))
}

pub fn find_ready_channel(
    client: &FnnClient,
    peer_pubkey: &str,
) -> Result<Option<Channel>, Box<dyn Error>> {
    let channels = client.list_channels(Some(peer_pubkey), true)?;

    Ok(channels
        .into_iter()
        .find(|channel| channel.state == "ChannelReady"))
}

pub fn wait_for_ready_channel(
    client: &FnnClient,
    peer_pubkey: &str,
) -> Result<Option<Channel>, Box<dyn Error>> {
    for _ in 0..20 {
        if let Some(channel) = find_ready_channel(client, peer_pubkey)? {
            return Ok(Some(channel));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(None)
}

pub fn wait_for_payment_success(
    client: &FnnClient,
    payment_hash: &str,
) -> Result<PaymentResponse, Box<dyn Error>> {
    let mut last_response = client.get_payment(payment_hash)?;

    for _ in 0..20 {
        match last_response.status {
            PaymentStatus::Success => return Ok(last_response),
            PaymentStatus::Failed
            | PaymentStatus::Cancelled
            | PaymentStatus::Canceled
            | PaymentStatus::Expired => {
                return Err(format!(
                    "payment {} ended in status {}",
                    payment_hash,
                    format_status(&last_response.status)
                )
                .into());
            }
            _ => {
                std::thread::sleep(std::time::Duration::from_secs(1));
                last_response = client.get_payment(payment_hash)?;
            }
        }
    }

    Ok(last_response)
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

pub fn parse_amount_to_u128(value: &str) -> Result<u128, Box<dyn Error>> {
    if let Some(hex) = value.strip_prefix("0x") {
        return Ok(u128::from_str_radix(hex, 16)?);
    }
    Ok(value.parse::<u128>()?)
}

pub fn format_final_summary(summary: &PaymentSummary) -> String {
    [
        format!("channelID: {}", summary.channel_id),
        format!("remoteBalance: {}", summary.remote_balance),
        format!("invoiceID: {}", summary.invoice_id),
        format!("amountPaid: {}", summary.amount_paid),
        format!("recipientPubkey: {}", summary.recipient_pubkey),
        format!("paymentHash: {}", summary.payment_hash),
    ]
    .join("\n")
}
