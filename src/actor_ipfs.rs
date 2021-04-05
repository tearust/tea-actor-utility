use wascc_actor::prelude::*;
// use prost::Message;
// use crate::tpm_provider_proto::*;
use crate::actor_util::get_public_key_from_bytes;
use crate::vmh::get_outbound_sequence;
use tea_codec;
use tea_codec::error::TeaError;
use tea_codec::ipfs_codec::{BlockPutRequest, DhtProvideRequest, OP_DHT_PROV};
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::vmh;
use vmh_codec::IPFS_OUTER_PROVIDER_NAME;

pub fn ipfs_block_put(data: &[u8], pin: bool) -> anyhow::Result<(String, u64)> {
    let ipfs_res_bytes = call_ipfs_provider(
        tea_codec::ipfs_codec::OP_BLOCK_PUT.into(),
        serialize(BlockPutRequest {
            data: data.to_vec(),
            pin,
        })
        .map_err(|e| anyhow::anyhow!("{}", e))?,
    )?;

    // info!("block put profile received bytes: {:?}", &ipfs_res_bytes);
    let ipfs_res: tea_codec::ipfs_codec::BlockPutResponse = deserialize(ipfs_res_bytes.as_slice())
        .map_err(|e| TeaError::DeserializeError(e.to_string()))?;
    // info!("block put profile received response {:?}", &ipfs_res);
    Ok((ipfs_res.key, ipfs_res.size))
}

pub fn ipfs_get(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = call_ipfs_provider(
        tea_codec::ipfs_codec::OP_GET.into(),
        cid.as_bytes().to_vec(),
    )?;
    Ok(res)
}

pub fn ipfs_block_get(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = call_ipfs_provider(
        tea_codec::ipfs_codec::OP_BLOCK_GET.into(),
        cid.as_bytes().to_vec(),
    )?;
    Ok(res)
}

pub fn ipfs_block_get_async(cid: &str) -> anyhow::Result<Vec<u8>> {
    let res = call_ipfs_provider(
        tea_codec::ipfs_codec::OP_BLOCK_GET_ASYNC.into(),
        cid.as_bytes().to_vec(),
    )?;
    Ok(res)
}

pub fn ipfs_is_block_local(cid: &str) -> anyhow::Result<bool> {
    let res_bytes = call_ipfs_provider(
        tea_codec::ipfs_codec::OP_IS_BLOCK_LOCAL.into(),
        cid.as_bytes().to_vec(),
    )?;

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
    let res = call_ipfs_provider(tea_codec::ipfs_codec::OP_ID.into(), vec![])?;
    String::from_utf8(res).map_err(|e| e.into())
}

pub fn ipfs_swarm_peers() -> anyhow::Result<Vec<String>> {
    let res = call_ipfs_provider(tea_codec::ipfs_codec::OP_SWARM_PEERS.into(), vec![])?;
    Ok(codec::deserialize(&res).map_err(|e| anyhow::anyhow!("{}", e))?)
}

pub fn announce_as_provider(req: &DhtProvideRequest) -> anyhow::Result<String> {
    let deployment_id_bytes = call_ipfs_provider(
        OP_DHT_PROV.into(),
        codec::serialize(req).map_err(|e| anyhow::anyhow!("{}", e))?,
    )?;
    Ok(String::from_utf8(deployment_id_bytes)?)
}

pub fn generate_deployment_id(key_bytes: Vec<u8>) -> anyhow::Result<String> {
    let pubkey_bytes = get_public_key_from_bytes(&key_bytes)?;
    let deployment_id =
        announce_as_provider(&DhtProvideRequest::PinnerPubKey(pubkey_bytes.to_vec()))
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(deployment_id)
}

pub fn call_ipfs_provider(operation: String, msg: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                ref_seq: get_outbound_sequence()?,
                provider: IPFS_OUTER_PROVIDER_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::ProviderOperationRequest(
                    vmh::ProviderOperationRequest {
                        actor: "".into(),
                        operation,
                        msg,
                    },
                )),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}
