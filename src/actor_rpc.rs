use crate::vmh::get_outbound_sequence;
use prost::Message;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{rpc, vmh},
};
use vmh_codec::{ADAPTER_RPC_CHANNEL_NAME, LAYER1_RPC_CHANNEL_NAME};
use wascc_actor::prelude::*;

pub fn ipfs_block_get(
    cid: String,
    wait_locally: bool,
    return_if_not_exist: bool,
) -> anyhow::Result<Vec<u8>> {
    let rpc_res_bytes = call_adapter_rpc(rpc::AdapterGeneralRequest {
        msg: Some(rpc::adapter_general_request::Msg::IpfsBlockGetRequest(
            rpc::IpfsBlockGetRequest {
                cid: cid.clone(),
                wait_locally,
                return_if_not_exist,
            },
        )),
    })?;
    let res = rpc::AdapterGeneralResponse::decode(rpc_res_bytes.as_slice())?;
    if let Some(rpc::adapter_general_response::Msg::IpfsBlockGetResponse(res)) = res.msg {
        if !cid.eq(&res.cid) {
            return Err(anyhow::anyhow!(
                "block get result got different cid: expect is {}, actual is {}",
                &cid,
                &res.cid
            ));
        }
        return Ok(res.payload);
    }
    Err(anyhow::anyhow!("unknown response: {:?}", res))
}

pub fn layer1_add_new_node(tea_id: Vec<u8>) -> anyhow::Result<()> {
    let rpc_res_bytes = call_layer1_rpc(rpc::Layer1GeneralRequest {
        msg: Some(rpc::layer1_general_request::Msg::AddNewNodeRequest(
            rpc::AddNewNodeRequest {
                tea_id: tea_id.clone(),
            },
        )),
    })?;
    let res = rpc::Layer1GeneralResponse::decode(rpc_res_bytes.as_slice())?;
    if let Some(rpc::layer1_general_response::Msg::AddNewNodeResponse(res)) = res.msg {
        if !tea_id.eq(&res.tea_id) {
            return Err(anyhow::anyhow!(
                "layer1 add new node got different tea id: expect is {:?}, actual is {:?}",
                &tea_id,
                &res.tea_id
            ));
        }
        return Ok(());
    }
    Err(anyhow::anyhow!("unknown response: {:?}", res))
}

pub fn call_adapter_rpc(req: rpc::AdapterGeneralRequest) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                ref_seq: get_outbound_sequence()?,
                channel: ADAPTER_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::AdapterGeneralRequest(req)),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn call_layer1_rpc(req: rpc::Layer1GeneralRequest) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                ref_seq: get_outbound_sequence()?,
                channel: LAYER1_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::Layer1GeneralRequest(req)),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}
