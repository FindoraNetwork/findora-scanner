use {
    bech32::{self, FromBase32, ToBase32},
    ruc::*,
    zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey},
};

#[allow(unused)]
#[inline(always)]
pub fn public_key_to_base64(key: &XfrPublicKey) -> String {
    base64::encode_config(ZeiFromToBytes::zei_to_bytes(key), base64::URL_SAFE)
}

#[allow(unused)]
#[inline(always)]
pub fn public_key_from_base64(pk: &str) -> Result<XfrPublicKey> {
    base64::decode_config(pk, base64::URL_SAFE)
        .c(d!())
        .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
}

#[allow(unused)]
#[inline(always)]
pub fn bech32_encode<T: AsRef<[u8]> + ToBase32>(input: &T) -> String {
    bech32::encode("fra", input.to_base32()).unwrap()
}

#[allow(unused)]
#[inline(always)]
pub fn bech32_decode(input: &str) -> Result<Vec<u8>> {
    bech32::decode(input)
        .c(d!())
        .and_then(|(_, data)| Vec::<u8>::from_base32(&data).c(d!()))
}

#[allow(unused)]
#[inline(always)]
pub fn public_key_to_bech32(key: &XfrPublicKey) -> String {
    bech32_encode(&XfrPublicKey::zei_to_bytes(key))
}

#[allow(unused)]
#[inline(always)]
pub fn public_key_from_bech32(addr: &str) -> Result<XfrPublicKey> {
    bech32_decode(addr)
        .c(d!())
        .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
}

pub fn pubkey_to_fra_address(pubkey: &str) -> Result<String> {
    let pk = public_key_from_base64(pubkey).unwrap();
    let address = public_key_to_bech32(&pk);

    Ok(address)
}
