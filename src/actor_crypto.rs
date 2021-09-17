use prost::Message;
use vmh_codec::message::{encode_protobuf, structs_proto::crypto};
use wascc_actor::untyped;

const CAPABILITY: &'static str = "tea:crypto";

pub fn generate(key_type: String) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let req = crypto::KeyGenerationRequest { key_type };
    let res = crypto::KeyGenerationResponse::decode(
        untyped::default()
            .call(CAPABILITY, "GenerateKeyPair", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok((res.public_key, res.private_key))
}

pub fn sign(key_type: String, private_key: Vec<u8>, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crypto::SignRequest {
        key_type,
        private_key,
        data,
    };
    let res = crypto::SignResponse::decode(
        untyped::default()
            .call(CAPABILITY, "Sign", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.signature)
}

pub fn verify(
    key_type: String,
    public_key: Vec<u8>,
    data: Vec<u8>,
    signature: Vec<u8>,
) -> anyhow::Result<bool> {
    let req = crypto::VerifyRequest {
        key_type,
        data,
        public_key,
        signature,
    };
    let res = crypto::VerifyResponse::decode(
        untyped::default()
            .call(CAPABILITY, "Verify", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.result)
}

pub fn shamir_share(n: u8, k: u8, data: Vec<u8>) -> anyhow::Result<Vec<Vec<u8>>> {
    let req = crypto::ShamirShareRequest {
        n: n as u32,
        k: k as u32,
        data,
    };
    let res = crypto::ShamirShareResponse::decode(
        untyped::default()
            .call(CAPABILITY, "ShamirShare", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.slices)
}

pub fn shamir_recovery(k: u8, slices: Vec<Vec<u8>>) -> anyhow::Result<Vec<u8>> {
    let req = crypto::ShamirRecoveryRequest {
        k: k as u32,
        slices,
    };
    let res = crypto::ShamirRecoveryResponse::decode(
        untyped::default()
            .call(CAPABILITY, "ShamirRecovery", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.data)
}

pub fn generate_multi_sig_asset(
    k: u8,
    public_keys: Vec<Vec<u8>>,
    key_type: String,
) -> anyhow::Result<String> {
    let req = crypto::GenerateMultiSigAssetRequest {
        key_type,
        public_keys,
        k: k as u32,
    };
    let res = crypto::GenerateMultiSigAssetResponse::decode(
        untyped::default()
            .call(CAPABILITY, "GenerateMultiSigAsset", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.address)
}

pub fn combine_to_witness(
    k: u8,
    public_keys: Vec<Vec<u8>>,
    signatures: Vec<Vec<u8>>,
    key_type: String,
) -> anyhow::Result<Vec<Vec<u8>>> {
    let req = crypto::CombineToWitnessRequest {
        key_type,
        public_keys,
        signatures,
        k: k as u32,
    };
    let res = crypto::CombineToWitnessResponse::decode(
        untyped::default()
            .call(CAPABILITY, "CombineToWitness", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.witness)
}

pub fn generate_aes_key() -> anyhow::Result<Vec<u8>> {
    let req = crypto::GenerateAesKeyRequest {};
    let res = crypto::GenerateAesKeyResponse::decode(
        untyped::default()
            .call(CAPABILITY, "GenerateAesKey", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.key)
}

pub fn aes_encrypt(key: Vec<u8>, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crypto::AesEncryptRequest { key, data };
    let res = crypto::AesEncryptResponse::decode(
        untyped::default()
            .call(CAPABILITY, "AesEncrypt", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.encrypted_data)
}

pub fn aes_decrypt(key: Vec<u8>, encrypted_data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crypto::AesDecryptRequest {
        key,
        encrypted_data,
    };
    let res = crypto::AesDecryptResponse::decode(
        untyped::default()
            .call(CAPABILITY, "AesDecrypt", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.data)
}

pub fn sha256(content: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crypto::ShaRequest {
        sha_type: "sha256".to_string(),
        content,
    };
    let res = crypto::ShaResponse::decode(
        untyped::default()
            .call(CAPABILITY, "Sha", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.hash)
}

pub fn public_key_from_ss58(address: &str) -> anyhow::Result<Vec<u8>> {
    let req = crypto::FromSs58AddressRequest {
        address: address.to_string(),
    };
    let res = crypto::FromSs58AddressResponse::decode(
        untyped::default()
            .call(CAPABILITY, "FromSS58", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.result)
}

pub fn public_key_to_ss58(public_key: &[u8]) -> anyhow::Result<String> {
    let res = crypto::ToSs58AddressResponse::decode(
        untyped::default()
            .call(
                CAPABILITY,
                "ToSS58",
                encode_protobuf(crypto::ToSs58AddressRequest {
                    public_key: public_key.to_vec(),
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.address)
}

pub fn generate_rsa_keypair(bit_size: u32) -> anyhow::Result<(String, String)> {
    let res = crypto::RsaKeyPairPemPcsk1Response::decode(
        untyped::default()
            .call(
                CAPABILITY,
                "GenerateRsaPkcs1",
                encode_protobuf(crypto::RsaKeyPairPemPcsk1Request { bits: bit_size })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok((res.public_key, res.private_key))
}

pub fn rsa_encrypt(pub_key: Vec<u8>, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let res = crypto::RsaEncryptResponse::decode(
        untyped::default()
            .call(
                CAPABILITY,
                "RsaEncrypt",
                encode_protobuf(crypto::RsaEncryptRequest {
                    public_key_pkcs1: pub_key,
                    msg: data,
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.result)
}

pub fn rsa_decrypt(key: Vec<u8>, encrypted_data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let res = crypto::RsaDecryptResponse::decode(
        untyped::default()
            .call(
                CAPABILITY,
                "RsaDecrypt",
                encode_protobuf(crypto::RsaDecryptRequest {
                    private_key_pkcs1: key,
                    msg: encrypted_data,
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.result)
}
