use prost::Message;
use vmh_codec::message::encode_protobuf;
use wascc_actor::untyped;

const CAPABILITY: &'static str = "tea:crypto";

pub fn generate(key_type: String) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let req = crate::crypto_proto::KeyGenerationRequest { key_type };
    let res = crate::crypto_proto::KeyGenerationResponse::decode(
        untyped::default()
            .call(CAPABILITY, "GenerateKeyPair", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok((res.public_key, res.private_key))
}

pub fn sign(key_type: String, private_key: Vec<u8>, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::SignRequest {
        key_type,
        private_key,
        data,
    };
    let res = crate::crypto_proto::SignResponse::decode(
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
    let req = crate::crypto_proto::VerifyRequest {
        key_type,
        data,
        public_key,
        signature,
    };
    let res = crate::crypto_proto::VerifyResponse::decode(
        untyped::default()
            .call(CAPABILITY, "Verify", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.result)
}

pub fn shamir_share(n: u8, k: u8, data: Vec<u8>) -> anyhow::Result<Vec<Vec<u8>>> {
    let req = crate::crypto_proto::ShamirShareRequest {
        n: n as u32,
        k: k as u32,
        data,
    };
    let res = crate::crypto_proto::ShamirShareResponse::decode(
        untyped::default()
            .call(CAPABILITY, "ShamirShare", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.slices)
}

pub fn shamir_recovery(k: u8, slices: Vec<Vec<u8>>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::ShamirRecoveryRequest {
        k: k as u32,
        slices,
    };
    let res = crate::crypto_proto::ShamirRecoveryResponse::decode(
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
    let req = crate::crypto_proto::GenerateMultiSigAssetRequest {
        key_type,
        public_keys,
        k: k as u32,
    };
    let res = crate::crypto_proto::GenerateMultiSigAssetResponse::decode(
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
    let req = crate::crypto_proto::CombineToWitnessRequest {
        key_type,
        public_keys,
        signatures,
        k: k as u32,
    };
    let res = crate::crypto_proto::CombineToWitnessResponse::decode(
        untyped::default()
            .call(CAPABILITY, "CombineToWitness", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.witness)
}

pub fn generate_aes_key() -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::GenerateAesKeyRequest {};
    let res = crate::crypto_proto::GenerateAesKeyResponse::decode(
        untyped::default()
            .call(CAPABILITY, "GenerateAesKey", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.key)
}

pub fn aes_encrypt(key: Vec<u8>, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::AesEncryptRequest { key, data };
    let res = crate::crypto_proto::AesEncryptResponse::decode(
        untyped::default()
            .call(CAPABILITY, "AesEncrypt", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.encrypted_data)
}

pub fn aes_decrypt(key: Vec<u8>, encrypted_data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::AesDecryptRequest {
        key,
        encrypted_data,
    };
    let res = crate::crypto_proto::AesDecryptResponse::decode(
        untyped::default()
            .call(CAPABILITY, "AesDecrypt", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.data)
}

pub fn construct_polkadot_tx(
    to_public_key: Vec<u8>,
    private_key: Vec<u8>,
    amount: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::ConstructTxRequest {
        msg: Some(
            crate::crypto_proto::construct_tx_request::Msg::PolkadotConstructExtrinsicRequest(
                crate::crypto_proto::PolkadotConstructExtrinsicRequest {
                    to_public_key,
                    amount,
                    private_key,
                },
            ),
        ),
    };
    let res = crate::crypto_proto::ConstructTxResponse::decode(
        untyped::default()
            .call(CAPABILITY, "ConstructTx", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.raw_transaction)
}

pub fn send_tx(key_type: String, raw_transaction: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::SendTxRequest {
        key_type,
        raw_transaction,
    };
    let res = crate::crypto_proto::SendTxResponse::decode(
        untyped::default()
            .call(CAPABILITY, "SendTx", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.hash)
}

pub fn sha256(content: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let req = crate::crypto_proto::ShaRequest {
        sha_type: "sha256".to_string(),
        content,
    };
    let res = crate::crypto_proto::ShaResponse::decode(
        untyped::default()
            .call(CAPABILITY, "Sha", encode_protobuf(req)?)
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    Ok(res.hash)
}
