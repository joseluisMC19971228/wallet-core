// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::chains::thorchain::test_cases::send_fd0445af::{
    signing_input, JSON_SIGNING_SIGNATURE, JSON_SIGNING_SIGNATURE_JSON, JSON_TX, PRIVATE_KEY,
};
use crate::chains::thorchain::THORCHAIN_COIN_TYPE;
use tw_any_coin::ffi::tw_any_signer::tw_any_signer_sign;
use tw_coin_entry::error::SigningErrorType;
use tw_encoding::hex::{DecodeHex, ToHex};
use tw_memory::test_utils::tw_data_helper::TWDataHelper;
use tw_proto::Cosmos::Proto;
use tw_proto::{deserialize, serialize};

#[test]
fn test_any_signer_sign_thorchain() {
    let input = Proto::SigningInput {
        signing_mode: Proto::SigningMode::JSON,
        private_key: PRIVATE_KEY.decode_hex().unwrap().into(),
        ..signing_input()
    };

    let input_data = TWDataHelper::create(serialize(&input).unwrap());

    let output =
        TWDataHelper::wrap(unsafe { tw_any_signer_sign(input_data.ptr(), THORCHAIN_COIN_TYPE) })
            .to_vec()
            .expect("!tw_any_signer_sign returned nullptr");

    let output: Proto::SigningOutput = deserialize(&output).unwrap();
    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    // https://viewblock.io/thorchain/tx/FD0445AFFC4ED9ACCB7B5D3ADE361DAE4596EA096340F1360F1020381EA454AF
    assert_eq!(output.json, JSON_TX);
    assert_eq!(output.signature.to_hex(), JSON_SIGNING_SIGNATURE);
    assert_eq!(output.signature_json, JSON_SIGNING_SIGNATURE_JSON);
}
