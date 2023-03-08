use {
    bech32::{self, FromBase32, ToBase32},
    ruc::*,
    zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey},
};

#[allow(unused)]
#[inline(always)]
pub fn public_key_to_base64(key: &XfrPublicKey) -> String {
    base64::encode_config(&ZeiFromToBytes::zei_to_bytes(key), base64::URL_SAFE)
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
fn bech32_encode<T: AsRef<[u8]> + ToBase32>(input: &T) -> String {
    bech32::encode("fra", input.to_base32()).unwrap()
}

#[allow(unused)]
#[inline(always)]
fn bech32_decode(input: &str) -> Result<Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_public_key_to_bech32() -> Result<()> {
        let addr_bech32 = "fra1xklsu8an2y4yd2e8kc43xspx54vrppp4l8fhtpey4n9z0kvmw2tqf76l2c";
        let pk_b64 = "Nb8OH7NRKkarJ7YrE0AmpVgwhDX503WHJKzKJ9mbcpY=";

        let pk1 = public_key_from_bech32(addr_bech32).unwrap();
        let pk2 = public_key_from_base64(pk_b64).unwrap();
        assert_eq!(pk1, pk2);

        let addr_bech32_1 = public_key_to_bech32(&pk2);
        assert_eq!(addr_bech32, addr_bech32_1);

        let pk_b64_1 = public_key_to_base64(&pk1);
        assert_eq!(pk_b64, pk_b64_1);

        Ok(())
    }

    #[tokio::test]
    async fn test_asset_code() -> Result<()> {
        let asset_addr = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        let asset_raw: Vec<u8> = vec![0u8; 32];
        let res_code = base64::decode(asset_addr).unwrap();
        assert_eq!(asset_raw, res_code);

        let res_addr = base64::encode(&asset_raw);
        assert_eq!(asset_addr, res_addr);
        Ok(())
    }

    #[tokio::test]
    async fn pk_to_bech32() -> Result<()> {
        let target = "fra1rkvlrs8j8y7rlud9qh6ndg5nr4ag7ar4640dr8h0ys6zfrwv25as42zptu";
        let pk_b64 = "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs=";
        let pk = public_key_from_base64(pk_b64).unwrap();
        let addr_bech32 = public_key_to_bech32(&pk);
        assert_eq!(addr_bech32, target);
        Ok(())
    }
}
