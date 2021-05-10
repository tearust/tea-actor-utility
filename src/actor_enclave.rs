use tea_codec::{OP_EPHEMERAL_PRI_KEY, OP_EPHEMERAL_PUB_KEY, OP_GET_TEA_ID};
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

#[cfg(feature = "tpm")]
pub fn get_my_signed_pcrs() -> anyhow::Result<Vec<u8>> {
    let res = untyped::default()
        .call(CAPABILITY, "GetSignedPcrBytes", Vec::new())
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(res)
}
