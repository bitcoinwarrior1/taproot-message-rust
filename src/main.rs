use bitcoin::{Address, Amount, Network, OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Witness};
use bitcoin::absolute::LockTime;
use bitcoin::blockdata::script::Builder;
use bitcoin::key::UntweakedPublicKey;
use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
use bitcoin::sighash::SighashCache;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::transaction::Version;

fn main() {
    let msg = b"YOUR_MESSAGE";
    let custom_script = Builder::new()
        .push_slice(msg)
        .into_script();

    let secp = Secp256k1::new();
    let internal_key = SecretKey::from_slice(&[0x01; 32]).unwrap();
    let internal_pubkey = PublicKey::from_secret_key(&secp, &internal_key);

    // println!("{}", custom_script);
    // println!("{}", internal_pubkey);

    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, custom_script.clone())
        .unwrap()
        .finalize(&secp, UntweakedPublicKey::from(internal_pubkey))
        .unwrap();

    // println!("{:?}", taproot_spend_info);

    let address = Address::p2tr_tweaked(taproot_spend_info.output_key(), Network::Bitcoin);
    // println!("Taproot Address with embedded script: {}", address);

    let tx_in = TxIn {
        previous_output: OutPoint::null(),
        script_sig: custom_script,
        sequence: Sequence(0),
        witness: Witness::default(),
    };

    // println!("{:?}", tx_in);
    // println!("{}", address.script_pubkey());

    let tx_out = TxOut {
        value: Amount::ZERO,
        script_pubkey: address.script_pubkey(),
    };

    // println!("{:?}", tx_out);

    let mut tx = Transaction {
        version: Version::ONE,
        lock_time: LockTime::ZERO,
        input: vec![tx_in],
        output: vec![tx_out],
    };

    // println!("{:?}", tx);

    // Sign the transaction using a dummy private key
    let mut sighash_cache = SighashCache::new(&mut tx);
    let sighash = sighash_cache.segwit_signature_hash(
        0,
        &Script::new(),
        1000,
        bitcoin::EcdsaSighashType::All,
    ).unwrap();
    let signature = secp.sign_ecdsa(&sighash, &internal_key);

    // Add the signature and witness elements
    tx.input[0].witness.push(signature.serialize_der().to_vec());
    tx.input[0].witness.push(vec![0x01]); // Control block for Taproot
    tx.input[0].witness.push(custom_script.to_bytes());
    println!("{:?}", "signed transaction: \n");
    println!("{:?}", tx);

}


