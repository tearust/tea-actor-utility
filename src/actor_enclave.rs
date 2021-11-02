use prost::Message;
use tea_codec::{
    OP_EPHEMERAL_PRI_KEY, OP_EPHEMERAL_PUB_KEY, OP_GET_TEA_ID, OP_NITRO_GEN_RANDOM,
    OP_NITRO_GEN_UUID,
};
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::nitro;
use wascc_actor::prelude::*;

#[cfg(feature = "tpm")]
const CAPABILITY: &'static str = "tea:tpm";
#[cfg(feature = "nitro")]
const CAPABILITY: &'static str = "tea:nitro";

pub fn get_my_tea_id() -> anyhow::Result<Vec<u8>> {
    let res_vec = untyped::default()
        .call(CAPABILITY, OP_GET_TEA_ID, vec![])
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    if res_vec.len() == 0 {
        Err(anyhow::anyhow!("Not init tea id"))
    } else {
        Ok(res_vec)
    }
}

pub fn get_my_ephemeral_id() -> anyhow::Result<Vec<u8>> {
    let res_vec = untyped::default()
        .call(CAPABILITY, OP_EPHEMERAL_PUB_KEY, vec![])
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    if res_vec.len() == 0 {
        Err(anyhow::anyhow!("Not init ephemeral public key"))
    } else {
        Ok(res_vec)
    }
}

pub fn get_my_ephemeral_key() -> anyhow::Result<Vec<u8>> {
    let res_vec = untyped::default()
        .call(CAPABILITY, OP_EPHEMERAL_PRI_KEY, vec![])
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    if res_vec.len() == 0 {
        Err(anyhow::anyhow!("Not init ephemeral key"))
    } else {
        Ok(res_vec)
    }
}

pub fn generate_random(len: u32) -> anyhow::Result<Vec<u8>> {
    let res_vec = untyped::default()
        .call(
            CAPABILITY,
            OP_NITRO_GEN_RANDOM,
            encode_protobuf(nitro::GenRandomRequest { len })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let res = nitro::GenRandomResponse::decode(res_vec.as_slice())?;
    Ok(res.data)
}

#[cfg(feature = "nitro")]
pub fn generate_uuid() -> anyhow::Result<String> {
    let res_vec = untyped::default()
        .call(CAPABILITY, OP_NITRO_GEN_UUID, vec![])
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let res = nitro::GenUuidResponse::decode(res_vec.as_slice())?;
    Ok(res.id)
}

#[cfg(feature = "tpm")]
pub fn get_my_signed_pcrs() -> anyhow::Result<Vec<u8>> {
    let res = untyped::default()
        .call(CAPABILITY, "GetSignedPcrBytes", Vec::new())
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(res)
}
