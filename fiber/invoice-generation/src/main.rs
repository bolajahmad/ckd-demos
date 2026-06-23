use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::{
    client::{Channel, ChannelApi, InvoiceApi, NodeApi, PaymentApi, PeerApi},
    utils::{
        find_ready_channel, format_final_summary, load_node, parse_amount_to_u128,
        wait_for_payment_success, wait_for_ready_channel,
    },
};

mod client;
mod utils;

const NODES_DIR: &str = "nodes";
const OUT_FILE: &str = "out.txt";
const CHANNEL_FUNDING_SHANNONS: &str = "10000000000";
const INVOICE_AMOUNT_SHANNONS: &str = "1000";
const INVOICE_DESCRIPTION: &str = "invoice-generated-by-bob";
const INVOICE_EXPIRY_SECONDS: u64 = 3600;

#[derive(Debug, Clone, Deserialize)]
struct NodeConfigFile {
    rpc: RpcConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcConfig {
    listening_addr: String,
}

#[derive(Debug, Clone)]
struct ChannelSummaryReport {
    channel_id: String,
    remote_balance: String,
}

#[derive(Debug, Clone)]
struct PaymentSummary {
    channel_id: String,
    remote_balance: u128,
    invoice_id: String,
    amount_paid: u128,
    recipient_pubkey: String,
    payment_hash: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load `alice` and `bob` clients
    let alice_client = load_node(Path::new(NODES_DIR), "alice")?;
    let bob_client = load_node(Path::new(NODES_DIR), "bob")?;

    // Load node info of both `alice` and `bob` clients
    let _alice_info = alice_client.node_info()?;
    let bob_info = bob_client.node_info()?;

    // Connect `alice` to `bob` as peers
    // Only one peer is required to initiate the connection
    alice_client.connect_peer_by_pubkey(&bob_info.pubkey)?;

    // Load existing channel between `alice` and `bob` if exists,
    // otherwise open a new channel and wait for it to become ready
    let existing_channel: Option<Channel> = find_ready_channel(&alice_client, &bob_info.pubkey)?;
    let channel_summary = if let Some(channel) = existing_channel {
        println!("step channel_ready already_exists");
        ChannelSummaryReport {
            channel_id: channel.channel_id,
            remote_balance: channel.remote_balance,
        }
    } else {
        println!("step channel_open requested");

        let open_response: Option<String> =
            alice_client.open_channel(&bob_info.pubkey, CHANNEL_FUNDING_SHANNONS)?;

        if let Some(temp_id) = open_response {
            bob_client.accept_channel(&temp_id, CHANNEL_FUNDING_SHANNONS)?;
        }

        let ready_channel = wait_for_ready_channel(&alice_client, &bob_info.pubkey)?;
        let Some(channel) = ready_channel else {
            return Err("channel did not become ready".into());
        };

        ChannelSummaryReport {
            channel_id: channel.channel_id,
            remote_balance: channel.remote_balance,
        }
    };

    // Create invoice using `bob`
    let invoice_response = bob_client.new_invoice(
        INVOICE_AMOUNT_SHANNONS,
        INVOICE_DESCRIPTION,
        INVOICE_EXPIRY_SECONDS,
    )?;
    let parsed_invoice = bob_client.parse_invoice(&invoice_response.invoice_address)?;

    let parsed_invoice_envelope = &parsed_invoice.invoice;
    let invoice_attrs = &parsed_invoice_envelope.data.attrs;
    let recipient_pubkey = invoice_attrs
        .iter()
        .find_map(|attr| attr.payee_public_key.clone())
        .unwrap_or_else(|| bob_info.pubkey.clone());

    let amount_paid = parse_amount_to_u128(&parsed_invoice_envelope.amount)?;

    // Pay invoice using `alice`
    let send_payment_response = alice_client.send_payment(&invoice_response.invoice_address)?;
    let confirmed_payment =
        wait_for_payment_success(&alice_client, &send_payment_response.payment_hash)?;

    // Print transaction summary to output file
    let final_summary = PaymentSummary {
        channel_id: channel_summary.channel_id,
        remote_balance: parse_amount_to_u128(&channel_summary.remote_balance)?,
        invoice_id: invoice_response.invoice_address,
        amount_paid,
        recipient_pubkey,
        payment_hash: confirmed_payment.payment_hash,
    };

    fs::write(OUT_FILE, format_final_summary(&final_summary))?;
    println!("step write_out_file ok");
    println!("flow completed");

    Ok(())
}
