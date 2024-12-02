use crate::{tests, types::core};

#[test]
fn dogecoin_testnet_read_header_bin() {
    tests::setup();

    for header_file in tests::data::find_doge_bin_files("testnet/headers", "") {
        log::info!("process file {}", header_file.display());

        let _header: core::DogecoinHeader = tests::utilities::decode_from_bin_file(&header_file);
    }
}
