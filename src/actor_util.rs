use crate::encode_protobuf;
use crate::tpm_provider_proto::*;
use ed25519_dalek::Keypair;
use prost::Message;
use tea_codec::error::TeaError;
use wascc_actor::prelude::*;

pub fn generate_ed25519_keypair() -> anyhow::Result<Vec<u8>> {
    let key_bytes = untyped::default()
        .call("tea:tpm", "GenerateEd25519KeyPair", vec![])
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(key_bytes)
}

pub fn verify_ed25519_signature(
    public_key: Vec<u8>,
    msg: Vec<u8>,
    signature: Vec<u8>,
) -> anyhow::Result<bool> {
    // info!("==============inside util verify sig function \nsignautre hex is {}\npublic_keyhex{}\nmsghex{}",
    //   hex::encode(&signature),
    //   hex::encode(&public_key),
    //   hex::encode(&msg),
    // );

    let req = VerifySignatureRequest {
        signature,
        public_key,
        msg,
    };

    let buf: Vec<u8> =
        crate::encode_protobuf(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res = untyped::default()
        .call("tea:tpm", "Ed25519VerifySignature", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let result = VerifySignatureResponse::decode(res.as_slice())?;

    Ok(result.result)
}

pub fn sign_ed25519_message(message: &Vec<u8>, key: Option<Vec<u8>>) -> anyhow::Result<Vec<u8>> {
    // info!("enter sign_ed... in actor_util msg is {:?}", message);
    let msg = message.to_vec();
    let req = Ed25519SignRequest { msg, key };
    let buf: Vec<u8> =
        crate::encode_protobuf(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res = untyped::default()
        .call("tea:tpm", "Ed25519SignMessage", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(res)
}

pub fn verify_sr25519_signature(
    public_key: Vec<u8>,
    msg: Vec<u8>,
    signature: Vec<u8>,
) -> anyhow::Result<bool> {
    let req = VerifySignatureRequest {
        signature,
        public_key,
        msg,
    };

    let buf: Vec<u8> =
        crate::encode_protobuf(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res = untyped::default()
        .call("tea:tpm", "Sr25519VerifySignature", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let result = VerifySignatureResponse::decode(res.as_slice())?;

    Ok(result.result)
}

pub fn sign_sr25519_message(message: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let req = Sr25519SignRequest {
        msg: message.to_vec(),
        key: key.to_vec(),
    };
    let buf: Vec<u8> =
        crate::encode_protobuf(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res = untyped::default()
        .call("tea:tpm", "Sr25519SignMessage", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(res)
}

pub fn encode_response<T>(response: T) -> HandlerResult<Vec<u8>>
where
    T: prost::Message,
{
    let mut buf: Vec<u8> = Vec::with_capacity(response.encoded_len());
    response.encode(&mut buf)?;
    Ok(buf)
}

pub fn ras_keys_to_bytes(key: String) -> anyhow::Result<Vec<u8>> {
    let der_encoded =
        key.lines()
            .filter(|line| !line.starts_with("-"))
            .fold(String::new(), |mut data, line| {
                data.push_str(&line);
                data
            });
    let der_encoded = base64::decode(&der_encoded)?;
    Ok(der_encoded)
}

pub fn rsa_encrypt(rsa_pub_key: Vec<u8>, msg: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let encrypt_req = RsaEncryptRequest {
        public_key_pkcs1: rsa_pub_key,
        msg,
    };

    let buf = encode_protobuf(encrypt_req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res_bytes = untyped::default()
        .call("tea:tpm", "RsaEncrypt", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let enc_ekey = RsaEncryptResponse::decode(res_bytes.as_slice())?;
    // info!(
    //     "\n\n\n\n**The re-encrypted ekey is {}",
    //     hex::encode(&enc_ekey.result)
    // );
    Ok(enc_ekey.result)
}

pub fn rsa_decrypt(rsa_priv_key: Vec<u8>, encrypted_msg: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let decrypt_req = RsaDecryptRequest {
        private_key_pkcs1: rsa_priv_key,
        msg: encrypted_msg,
    };
    // info!("decrypt_req is {:?}", &decrypt_req);
    let buf = encode_protobuf(decrypt_req).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let res_bytes = untyped::default()
        .call("tea:tpm", "RsaDecrypt", buf)
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let dec_key = RsaDecryptResponse::decode(res_bytes.as_slice())?;
    // info!(
    //     "\n\n\n\n**The new original key is hex:{}\n\n\n\n",
    //     hex::encode(&dec_key.result)
    // );
    Ok(dec_key.result)
}

pub fn url_decode(url: &str) -> HandlerResult<String> {
    let value = url::Url::parse(url)?;
    Ok(value.to_string())
}

pub fn generate_rsa_keypair() -> anyhow::Result<crate::tpm_provider_proto::RsaKeyPairPemPcsk1> {
    //HandlerResult<actor_delegate_proto::DataRegisterResponse> {
    let res_rsa_key_pkcs1 = untyped::default()
        .call("tea:tpm", "GenerateRsaPkcs1", Vec::new())
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let rsa_key_pkcs1 =
        crate::tpm_provider_proto::RsaKeyPairPemPcsk1::decode(res_rsa_key_pkcs1.as_slice())?;

    // info!(
    //     "rsa_key_pkcs1 to string is \n{}\n{}",
    //     rsa_key_pkcs1.public_key, rsa_key_pkcs1.private_key
    // );
    Ok(rsa_key_pkcs1)
}

pub fn get_public_key_from_bytes(key_bytes: &[u8]) -> anyhow::Result<[u8; 32]> {
    let keypair = Keypair::from_bytes(&key_bytes)?;
    Ok(keypair.public.to_bytes())
}
