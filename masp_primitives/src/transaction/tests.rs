use ff::Field;
use rand_core::OsRng;

use super::{components::Amount, sighash::signature_hash, Transaction, TransactionData};
use crate::{constants::SPENDING_KEY_GENERATOR, redjubjub::PrivateKey};

#[test]
fn tx_read_write() {
    let data = &self::data::tx_read_write::TX_READ_WRITE;
    let mut rdr = &data[..];
    let tx = Transaction::read(&mut rdr).unwrap();
    assert_eq!(
        format!("{}", tx.txid()),
        "64f0bd7fe30ce23753358fe3a2dc835b8fba9c0274c4e2c54a6f73114cb55639"
    );

    let mut encoded = Vec::with_capacity(data.len());
    tx.write(&mut encoded).unwrap();
    assert_eq!(&data[..], &encoded[..]);
}

#[test]
fn tx_write_rejects_unexpected_joinsplit_pubkey() {
    // Succeeds without a JoinSplit pubkey
    assert!(TransactionData::new().freeze().is_ok());

    // Fails with an unexpected JoinSplit pubkey
    {
        let mut tx = TransactionData::new();
        tx.joinsplit_pubkey = Some([0; 32]);
        assert!(tx.freeze().is_err());
    }
}

#[test]
fn tx_write_rejects_unexpected_joinsplit_sig() {
    // Succeeds without a JoinSplit signature
    assert!(TransactionData::new().freeze().is_ok());

    // Fails with an unexpected JoinSplit signature
    {
        let mut tx = TransactionData::new();
        tx.joinsplit_sig = Some([0; 64]);
        assert!(tx.freeze().is_err());
    }
}

#[test]
fn tx_write_rejects_unexpected_binding_sig() {
    // Succeeds without a binding signature
    assert!(TransactionData::new().freeze().is_ok());

    // Fails with an unexpected binding signature
    {
        let mut rng = OsRng;
        let sk = PrivateKey(jubjub::Fr::random(&mut rng));
        let sig = sk.sign(b"Foo bar", &mut rng, SPENDING_KEY_GENERATOR);

        let mut tx = TransactionData::new();
        tx.binding_sig = Some(sig);
        assert!(tx.freeze().is_err());
    }
}

mod data;
#[test]
fn zip_0143() {
    for tv in self::data::zip_0143::make_test_vectors() {
        let mut rdr = &tv.tx[..];
        let tx = Transaction::read(&mut rdr).unwrap();
        let transparent_input = tv.transparent_input.map(|n| {
            (
                n as usize,
                &tv.script_code,
                tv.asset_type,
                tv.amount as u64,
            )
        });

        assert_eq!(
            signature_hash(&tx, tv.consensus_branch_id, tv.hash_type, transparent_input),
            tv.sighash
        );
    }
}

#[test]
fn zip_0243() {
    for tv in self::data::zip_0243::make_test_vectors() {
        let mut rdr = &tv.tx[..];
        let tx = Transaction::read(&mut rdr).unwrap();
        let transparent_input = tv.transparent_input.map(|n| {
            (
                n as usize,
                &tv.script_code,
                tv.asset_type,
                tv.amount as u64,
            )
        });

        assert_eq!(
            signature_hash(&tx, tv.consensus_branch_id, tv.hash_type, transparent_input),
            tv.sighash
        );
    }
}