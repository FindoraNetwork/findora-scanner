use bech32::ToBase32;
use ethereum::{LegacyTransaction, LegacyTransactionMessage};
use ethereum_types::{H160, H256};
use ruc::eg;
use sha3::{Digest, Keccak256};

pub fn recover_signer(transaction: &LegacyTransaction) -> Option<H160> {
    let mut sig = [0u8; 65];
    let mut msg = [0u8; 32];
    sig[0..32].copy_from_slice(&transaction.signature.r()[..]);
    sig[32..64].copy_from_slice(&transaction.signature.s()[..]);
    sig[64] = transaction.signature.standard_v();
    msg.copy_from_slice(&LegacyTransactionMessage::from(transaction.clone()).hash()[..]);

    let pubkey = secp256k1_ecdsa_recover(&sig, &msg).ok()?;
    Some(H160::from(H256::from_slice(
        Keccak256::digest(&pubkey).as_slice(),
    )))
}

/// Verify and recover a SECP256k1 ECDSA signature.
///
/// - `sig` is passed in RSV format. V should be either `0/1` or `27/28`.
/// - `msg` is the keccak-256 hash of the message.
///
/// Returns `Err` if the signature is bad, otherwise the 64-byte pubkey
/// (doesn't include the 0x04 prefix).
pub fn secp256k1_ecdsa_recover(sig: &[u8; 65], msg: &[u8; 32]) -> ruc::Result<[u8; 64]> {
    let rs = libsecp256k1::Signature::parse_standard_slice(&sig[0..64])
        .map_err(|_| eg!("Ecdsa signature verify error: bad RS"))?;
    let v = libsecp256k1::RecoveryId::parse(if sig[64] > 26 { sig[64] - 27 } else { sig[64] })
        .map_err(|_| eg!("Ecdsa signature verify error: bad V"))?;
    let pubkey = libsecp256k1::recover(&libsecp256k1::Message::parse(msg), &rs, &v)
        .map_err(|_| eg!("Ecdsa signature verify error: bad signature"))?;
    let mut res = [0u8; 64];
    res.copy_from_slice(&pubkey.serialize()[1..65]);
    Ok(res)
}

#[inline(always)]
pub fn bech32enc<T: AsRef<[u8]> + ToBase32>(input: &T) -> String {
    bech32::encode("fra", input.to_base32()).unwrap()
}

#[cfg(test)]
#[allow(missing_docs)]
mod test {
    use super::*;
    use crate::schema::EvmTx;
    use ruc::{d, RucResult};
    use zei::serialization::ZeiFromToBytes;
    use zei::xfr::sig::XfrPublicKey;

    #[test]
    fn test_recover_signer() {
        // ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4MSIsImdhc19wcmljZSI6IjB4MTc0ODc2ZTgwMCIsImdhc19saW1pdCI6IjB4NTIwOCIsImFjdGlvbiI6eyJDYWxsIjoiMHgyYWQzMjg0NmM2ZGQyZmZkM2VkYWRiZTUxY2Q1YWUwNGFhNWU1NzVlIn0sInZhbHVlIjoiMHg1NmJjNzVlMmQ2MzEwMDAwMCIsImlucHV0IjpbXSwic2lnbmF0dXJlIjp7InYiOjEwODIsInIiOiIweGY4YWVmN2Y4MDUzZDg5ZmVlMzk1MGM0ZDcwMjA4MGJmM2E4MDcyYmVkNWQ4NGEzYWYxOWEzNjAwODFiNjM2YTIiLCJzIjoiMHgyOTYyOTlhOGYyNDMwYjg2ZmQzZWI5NzZlYWJjNzMwYWMxY2ZiYmJlMzZlYjY5ZWFlMzM4Y2ZmMzNjNGE5OGMxIn19fX19
        let tx_str = "{\"signature\":null,\"function\":{\"Ethereum\":{\"Transact\":{\"nonce\":\"0x1\",\"gas_price\":\"0x174876e800\",\"gas_limit\":\"0x5208\",\"action\":{\"Call\":\"0x2ad32846c6dd2ffd3edadbe51cd5ae04aa5e575e\"},\"value\":\"0x56bc75e2d63100000\",\"input\":[],\"signature\":{\"v\":1082,\"r\":\"0xf8aef7f8053d89fee3950c4d702080bf3a8072bed5d84a3af19a360081b636a2\",\"s\":\"0x296299a8f2430b86fd3eb976eabc730ac1cfbbbe36eb69eae338cff33c4a98c1\"}}}}}";
        let evm_tx: EvmTx = serde_json::from_str(tx_str).unwrap();
        let signer = recover_signer(&evm_tx.function.ethereum.transact).unwrap();
        let addr = format!("{signer:?}");
        assert_eq!(addr, "0xa5225cbee5052100ec2d2d94aa6d258558073757");
    }

    #[test]
    fn test_convert_base64_to_bech32() {
        let pk = base64::decode_config(
            "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs=",
            base64::URL_SAFE,
        )
        .c(d!())
        .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
        .unwrap();
        let addr = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
        assert_eq!(
            addr,
            "fra1rkvlrs8j8y7rlud9qh6ndg5nr4ag7ar4640dr8h0ys6zfrwv25as42zptu"
        );
    }
}
