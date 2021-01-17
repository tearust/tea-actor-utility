use wascc_actor::prelude::*;
// use prost::Message;
// use crate::tpm_provider_proto::*;
use crate::actor_util::get_public_key_from_bytes;
use tea_codec;
use tea_codec::error::TeaError;
use tea_codec::ipfs_codec::{BlockPutRequest, DhtProvideRequest, OP_DHT_PROV};

pub fn ipfs_block_put(data: &[u8], pin: bool) -> anyhow::Result<(String, u64)> {
    let ipfs_res_bytes = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_BLOCK_PUT,
            codec::serialize(BlockPutRequest {
                data: data.to_vec(),
                pin,
            })
            .map_err(|e| TeaError::SerializeError(e.to_string()))?,
        )
        .map_err(|e| {
            error!("ipfs op_block_put error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;
    // info!("block put profile received bytes: {:?}", &ipfs_res_bytes);
    let ipfs_res: tea_codec::ipfs_codec::BlockPutResponse = deserialize(ipfs_res_bytes.as_slice())
        .map_err(|e| TeaError::DeserializeError(e.to_string()))?;
    // info!("block put profile received response {:?}", &ipfs_res);
    Ok((ipfs_res.key, ipfs_res.size))
}

pub fn ipfs_get(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_GET,
            cid.as_bytes().to_vec(),
        )
        .map_err(|e| {
            // error!("ipfs op_get error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;
    Ok(res)
}

pub fn ipfs_block_get(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_BLOCK_GET,
            cid.as_bytes().to_vec(),
        )
        .map_err(|e| {
            // error!("ipfs op_block_get error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;

    Ok(res)
    // let ret: BlockPutRequest = deserialize(&res)?;
    // Ok(ret.data)
}

pub fn ipfs_block_get_async(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_BLOCK_GET_ASYNC,
            cid.as_bytes().to_vec(),
        )
        .map_err(|e| {
            // error!("ipfs op_block_get_async error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;
    Ok(res)
}

pub fn ipfs_is_block_local(cid: &str) -> anyhow::Result<bool> {
    let res_bytes = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_IS_BLOCK_LOCAL,
            cid.as_bytes().to_vec(),
        )
        .map_err(|e| {
            // error!("ipfs op_is_block_local error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;
    let res = {
        let temp: tea_codec::ipfs_codec::IsBlockLocalResponse =
            deserialize(&res_bytes).map_err(|e| {
                // error!("ipfs op_is_block_local error {}", e);
                TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
            })?;
        if temp.error.is_empty() {
            temp.result
        } else {
            info!("IPFS IsBlockLocal error {}", temp.error);
            false
        }
    };
    Ok(res)
}

pub fn ipfs_id() -> anyhow::Result<String> {
    let res = untyped::default()
        .call("tea:ipfs", tea_codec::ipfs_codec::OP_ID, Vec::new())
        .map_err(|e| {
            // error!("ipfs id error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })
        .map_err(|e| {
            // error!("ipfs op_is_block_local error {}", e);
            TeaError::CommonError(format!("{}:{}-{}", file!(), line!(), e))
        })?;
    String::from_utf8(res).map_err(|e| e.into())
}

pub fn ipfs_swarm_peers() -> anyhow::Result<Vec<String>> {
    let res = untyped::default()
        .call(
            "tea:ipfs",
            tea_codec::ipfs_codec::OP_SWARM_PEERS,
            Vec::new(),
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(codec::deserialize(&res).map_err(|e| anyhow::anyhow!("{}", e))?)
}

pub fn announce_as_provider(req: &DhtProvideRequest) -> anyhow::Result<String> {
    let deployment_id_bytes = untyped::default()
        .call(
            "tea:ipfs",
            OP_DHT_PROV,
            codec::serialize(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?,
        )
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(String::from_utf8(deployment_id_bytes)?)
}

pub fn generate_deployment_id(key_bytes: Vec<u8>) -> anyhow::Result<String> {
    let pubkey_bytes = get_public_key_from_bytes(&key_bytes)?;
    let deployment_id =
        announce_as_provider(&DhtProvideRequest::PinnerPubKey(pubkey_bytes.to_vec()))
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(deployment_id)
}
