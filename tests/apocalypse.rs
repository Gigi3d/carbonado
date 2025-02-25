use std::fs::read;

use anyhow::Result;
use carbonado::{encode, scrub, utils::init_logging};
use ecies::utils::generate_keypair;
use log::{debug, info};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn contract() -> Result<()> {
    init_logging();

    act_of_god("tests/samples/contract.rgbc")?;

    Ok(())
}

#[wasm_bindgen_test]
fn wasm_contract() -> Result<()> {
    init_logging();

    act_of_god("tests/samples/contract.rgbc")?;

    Ok(())
}

#[test]
fn content() -> Result<()> {
    act_of_god("tests/samples/content.png")?;

    Ok(())
}

#[wasm_bindgen_test]
fn wasm_content() -> Result<()> {
    act_of_god("tests/samples/content.png")?;

    Ok(())
}

#[test]
fn code() -> Result<()> {
    act_of_god("tests/samples/code.tar")?;

    Ok(())
}

#[wasm_bindgen_test]
fn wasm_code() -> Result<()> {
    act_of_god("tests/samples/code.tar")?;

    Ok(())
}

fn act_of_god(path: &str) -> Result<()> {
    let input = read(path)?;
    let (_privkey, pubkey) = generate_keypair();
    info!("Encoding {path}...");
    let (orig_encoded, hash, padding, encode_info) = encode(&pubkey.serialize(), &input)?;
    debug!("Padding was {padding}. Encoding Info: {encode_info:#?}");
    let mut new_encoded = Vec::new();
    new_encoded.clone_from(&orig_encoded);

    info!("Scrubbing stream against hash: {hash}...");
    let orig_result = scrub(&orig_encoded, padding, hash.as_bytes());
    assert!(
        orig_result.is_err(),
        "Return error when there's no need to scrub"
    );

    new_encoded[0] ^= 64; // ⚡️

    info!("Scrubbing modified stream against hash: {hash}...");
    let new_result = scrub(&new_encoded, padding, hash.as_bytes())?;
    assert_eq!(
        new_result, orig_encoded,
        "Original and scrubbed data are the same"
    );
    info!("All good!");

    Ok(())
}
