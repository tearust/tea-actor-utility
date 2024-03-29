use ed25519_dalek::Keypair;
#[cfg(feature = "tpm")]
use prost::Message;
#[cfg(feature = "tpm")]
use tea_codec::error::TeaError;
use wascc_actor::prelude::*;

pub fn url_decode(url: &str) -> HandlerResult<String> {
	let value = url::Url::parse(url)?;
	Ok(value.to_string())
}

#[cfg(feature = "tpm")]
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
