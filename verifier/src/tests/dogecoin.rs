use crate::{tests, types::core};

#[test]
fn dogecoin_testnet_read_header_bin() {
    tests::setup();

    for header_file in tests::data::find_doge_bin_files("testnet/headers", "") {
        log::info!("process file {}", header_file.display());

        let _header: core::DogecoinHeader = tests::utilities::decode_from_bin_file(&header_file);
    }
}

use std::fs::read_to_string;

use alloc::format;
use ckb_jsonrpc_types::TransactionView;
use ckb_types::packed::WitnessArgs;
use serde_json::from_str as from_json_str;

use crate::{
    molecule::prelude::*,
    types::{
        packed::{SpvClient, SpvUpdate},
        prelude::*,
    },
};

#[test]
fn dogecoin_testnet_verify_new_client() {
    verify_new_client_common(
        "doge_update.json",
        1, // cell_dep_index
    );
}

fn verify_new_client_common(tx_file: &str, cell_dep_index: usize) {
    tests::setup();

    let path = tests::data::find_dogebin_bin_file("testnet", tx_file);
    let tx = read_to_string(path).unwrap();
    let tx: TransactionView = from_json_str(&tx).unwrap();

    let witnesses = tx.inner.witnesses;
    let witness_args = WitnessArgs::from_slice(witnesses[0].as_bytes()).unwrap();
    let spv_update_bin = witness_args.output_type().to_opt().unwrap().raw_data();
    let spv_update = SpvUpdate::from_slice(&spv_update_bin).unwrap();

    let client_bin = tx.inner.outputs_data[1].clone();
    let client = SpvClient::from_slice(client_bin.as_bytes()).unwrap();

    let cell_dep = tx.inner.cell_deps[cell_dep_index].out_point.clone();
    let path = tests::data::find_dogebin_bin_file(
        "testnet",
        format!("tx-0x{}.json", cell_dep.tx_hash).as_str(),
    );
    std::println!("path:{:?}", path);
    let previous_tx = read_to_string(path).unwrap();
    let previous_tx: TransactionView = from_json_str(&previous_tx).unwrap();
    let cell_dep_data_bin = &previous_tx.inner.outputs_data[cell_dep.index.value() as usize];
    let cell_dep_client = SpvClient::from_slice(cell_dep_data_bin.as_bytes()).unwrap();

    let mut cell_dep_client: core::SpvClient = cell_dep_client.unpack();
    cell_dep_client.id = client.id().into();
    let input_client = cell_dep_client.pack();
    let ret = input_client.verify_new_client(&client, spv_update, 128);

    assert!(ret.is_ok());
}
