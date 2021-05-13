use crate::vmh::get_outbound_sequence;
use prost::Message;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{rpc, vmh},
};
use vmh_codec::LAYER1_RPC_CHANNEL_NAME;
use wascc_actor::prelude::*;

pub fn commit_ra_result(req: rpc::CommitRaResultRequest) -> anyhow::Result<()> {
    call_layer1_rpc(rpc::Layer1GeneralRequest {
        msg: Some(rpc::layer1_general_request::Msg::CommitRaResultRequest(req)),
    })?;
    Ok(())
}

pub fn layer1_update_node_profile(req: rpc::TeaNodeUpdateProfileRequest) -> anyhow::Result<()> {
    call_layer1_rpc(rpc::Layer1GeneralRequest {
        msg: Some(rpc::layer1_general_request::Msg::UpdateNodeProfileRequest(
            req,
        )),
    })?;
    Ok(())
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

pub fn register_layer1_dispatcher(type_ids: Vec<u32>) -> anyhow::Result<()> {
    untyped::default()
        .call(
            tea_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_REG_LAYER1_DISPATCHER_MESSAGE,
            encode_protobuf(vmh::RegisterDispatcherRequest { type_ids })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}
