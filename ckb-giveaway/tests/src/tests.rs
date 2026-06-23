use crate::{
    blake160, build_sighash_lock_script, deploy_sighash_lock_deps, script_hash, sign_tx_sighash,
    verify_and_dump_failed_tx,
};
use ckb_testtool::ckb_crypto::secp::Generator;
use ckb_testtool::ckb_error::Error as CKBError;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;
use std::fs;
use std::path::PathBuf;

// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for more examples

// generated unit test for contract giveaway
#[test]
fn test_giveaway_create_cell() {
    let mut context = Context::default();
    let giveaway_out_point = context.deploy_cell_by_name("giveaway");
    let lock_deps = deploy_sighash_lock_deps(&mut context);

    let host_privkey = Generator::random_privkey();
    let host_pubkey_hash = blake160(&host_privkey.pubkey().expect("host pubkey").serialize());
    let host_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &host_pubkey_hash);

    let verifier_privkey = Generator::random_privkey();
    let verifier_pubkey_hash =
        blake160(&verifier_privkey.pubkey().expect("verifier pubkey").serialize());
    let verifier_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &verifier_pubkey_hash);

    let verifier_lock_hash = script_hash(&verifier_lock_script);
    let giveaway_type_script = context
        .build_script(&giveaway_out_point, verifier_lock_hash.as_bytes())
        .expect("giveaway type script");

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1_000)
            .lock(host_lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let giveaway_data =
        build_giveaway_data(&script_hash(&host_lock_script), &verifier_lock_hash);
    let output = CellOutput::new_builder()
        .capacity(500)
        .lock(host_lock_script.clone())
        .type_(Some(giveaway_type_script.clone()).pack())
        .build();

    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .output_data(Bytes::from(giveaway_data.clone()).pack())
        .cell_dep(lock_deps.secp_data_dep.clone())
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx_sighash(tx, &host_privkey);

    let cycles = verify_and_dump_failed_tx(&context, &tx, 10_000_000)
        .expect("giveaway creation should verify");

    let output = tx.outputs().get(0).expect("first output");
    let output_type = output.type_().to_opt().expect("type script");
    let output_data = tx
        .outputs_data()
        .get(0)
        .expect("first output data")
        .raw_data();

    assert_eq!(output.lock(), host_lock_script);
    assert_eq!(output_type, giveaway_type_script);
    assert_eq!(output_data.as_ref(), giveaway_data.as_slice());

    println!("consume cycles: {}", cycles);
    println!(
        "host lock hash: 0x{}",
        hex_string(script_hash(&host_lock_script).as_slice())
    );
    println!(
        "verifier lock hash: 0x{}",
        hex_string(verifier_lock_hash.as_slice())
    );
    println!(
        "giveaway type script hash: 0x{}",
        hex_string(giveaway_type_script.calc_script_hash().as_slice())
    );
    println!("giveaway data: 0x{}", hex_string(giveaway_data.as_slice()));
    let mock_tx = context.dump_tx(&tx).expect("dump tx info");
    let fixture_path = fixture_path("giveaway-create.json");
    fs::create_dir_all(
        fixture_path
            .parent()
            .expect("fixture parent directory"),
    )
    .expect("create fixtures dir");
    fs::write(
        fixture_path,
        serde_json::to_string_pretty(&mock_tx).expect("tx format json"),
    )
    .expect("write fixture tx");
    println!(
        "{}",
        serde_json::to_string_pretty(&mock_tx).expect("tx format json")
    );
}

#[test]
fn test_non_owner_update_rejected() {
    let mut context = Context::default();
    let giveaway_out_point = context.deploy_cell_by_name("giveaway");
    let lock_deps = deploy_sighash_lock_deps(&mut context);

    let host_privkey = Generator::random_privkey();
    let host_pubkey_hash = blake160(&host_privkey.pubkey().expect("host pubkey").serialize());
    let host_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &host_pubkey_hash);

    let attacker_privkey = Generator::random_privkey();
    let attacker_pubkey_hash =
        blake160(&attacker_privkey.pubkey().expect("attacker pubkey").serialize());
    let attacker_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &attacker_pubkey_hash);

    let verifier_privkey = Generator::random_privkey();
    let verifier_pubkey_hash =
        blake160(&verifier_privkey.pubkey().expect("verifier pubkey").serialize());
    let verifier_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &verifier_pubkey_hash);
    let verifier_lock_hash = script_hash(&verifier_lock_script);

    let giveaway_type_script = context
        .build_script(&giveaway_out_point, verifier_lock_hash.as_bytes())
        .expect("giveaway type script");

    let existing_data = build_giveaway_data(&script_hash(&host_lock_script), &verifier_lock_hash);
    let existing_cell = CellOutput::new_builder()
        .capacity(500)
        .lock(attacker_lock_script.clone())
        .type_(Some(giveaway_type_script.clone()).pack())
        .build();
    let existing_out_point = context.create_cell(existing_cell, Bytes::from(existing_data.clone()));

    let giveaway_input = CellInput::new_builder()
        .previous_output(existing_out_point)
        .build();

    let mut updated_data = existing_data.clone();
    let updated_prize = 700u64;
    updated_data[12..20].copy_from_slice(&updated_prize.to_le_bytes());

    let updated_output = CellOutput::new_builder()
        .capacity(700)
        .lock(host_lock_script.clone())
        .type_(Some(giveaway_type_script.clone()).pack())
        .build();

    let tx = TransactionBuilder::default()
        .input(giveaway_input)
        .output(updated_output)
        .output_data(Bytes::from(updated_data).pack())
        .cell_dep(lock_deps.secp_data_dep.clone())
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx_sighash(tx, &attacker_privkey);

    let err = verify_and_dump_failed_tx(&context, &tx, 10_000_000)
        .expect_err("non-owner update must fail");
    expect_validation_failure_code(&err, 4);
}

#[test]
fn test_non_owner_cancel_rejected() {
    let mut context = Context::default();
    let giveaway_out_point = context.deploy_cell_by_name("giveaway");
    let lock_deps = deploy_sighash_lock_deps(&mut context);

    let host_privkey = Generator::random_privkey();
    let host_pubkey_hash = blake160(&host_privkey.pubkey().expect("host pubkey").serialize());
    let host_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &host_pubkey_hash);

    let attacker_privkey = Generator::random_privkey();
    let attacker_pubkey_hash =
        blake160(&attacker_privkey.pubkey().expect("attacker pubkey").serialize());
    let attacker_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &attacker_pubkey_hash);

    let verifier_privkey = Generator::random_privkey();
    let verifier_pubkey_hash =
        blake160(&verifier_privkey.pubkey().expect("verifier pubkey").serialize());
    let verifier_lock_script =
        build_sighash_lock_script(&mut context, &lock_deps.lock_out_point, &verifier_pubkey_hash);
    let verifier_lock_hash = script_hash(&verifier_lock_script);

    let giveaway_type_script = context
        .build_script(&giveaway_out_point, verifier_lock_hash.as_bytes())
        .expect("giveaway type script");

    let existing_data = build_giveaway_data(&script_hash(&host_lock_script), &verifier_lock_hash);
    let existing_cell = CellOutput::new_builder()
        .capacity(500)
        .lock(attacker_lock_script.clone())
        .type_(Some(giveaway_type_script.clone()).pack())
        .build();
    let existing_out_point = context.create_cell(existing_cell, Bytes::from(existing_data));

    let giveaway_input = CellInput::new_builder()
        .previous_output(existing_out_point)
        .build();

    // Cancel transition: consume giveaway input, no giveaway group output.
    let tx = TransactionBuilder::default()
        .input(giveaway_input)
        .output(
            CellOutput::new_builder()
                .capacity(1_000)
                .lock(attacker_lock_script.clone())
                .build(),
        )
        .output_data(Bytes::new().pack())
        .cell_dep(lock_deps.secp_data_dep)
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx_sighash(tx, &attacker_privkey);

    let err = verify_and_dump_failed_tx(&context, &tx, 10_000_000)
        .expect_err("non-owner cancel must fail");
    expect_validation_failure_code(&err, 4);
}

fn fixture_path(file: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("fixtures");
    path.push(file);
    path
}

fn expect_validation_failure_code(err: &CKBError, expected_code: i8) {
    let text = format!("{err:?}");
    let marker_v1 = format!("ValidationFailure({expected_code})");
    let marker_v2 = format!("error code {expected_code}");
    assert!(
        text.contains(&marker_v1) || text.contains(&marker_v2),
        "expected validation failure code {expected_code}, got error: {text}"
    );
}

fn build_giveaway_data(host_lock_hash: &Byte32, verifier_lock_hash: &Byte32) -> Vec<u8> {
    let mut data = Vec::with_capacity(116);
    data.push(1);
    data.push(1);
    data.push(3);
    data.push(0);
    data.extend_from_slice(&1u64.to_le_bytes());
    data.extend_from_slice(&500u64.to_le_bytes());
    data.extend_from_slice(&[7u8; 32]);
    data.extend_from_slice(host_lock_hash.as_slice());
    data.extend_from_slice(verifier_lock_hash.as_slice());
    data
}

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}
