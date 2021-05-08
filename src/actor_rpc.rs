use crate::vmh::get_outbound_sequence;
use prost::Message;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{rpc, vmh},
};
use vmh_codec::{ADAPTER_RPC_CHANNEL_NAME, LAYER1_RPC_CHANNEL_NAME};
use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;

pub fn ipfs_info(peer_id: Option<String>) -> anyhow::Result<rpc::IpfsInfoResponse> {
    let rpc_res_bytes = call_adapter_rpc(rpc::AdapterClientRequest {
        msg: Some(rpc::adapter_client_request::Msg::IpfsInfoRequest(
            rpc::IpfsInfoRequest {
                peer_id: peer_id.unwrap_or("".into()),
            },
        )),
    })?;
    Ok(rpc::IpfsInfoResponse::decode(rpc_res_bytes.as_slice())?)
}

pub fn ipfs_block_get<F>(
    cid: String,
    reply_to: String,
    wait_locally: bool,
    return_if_not_exist: bool,
    callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(&BrokerMessage) -> anyhow::Result<()> + Sync + Send + 'static,
{
    call_adapter_rpc_async(
        move |uuid| {
            Ok(rpc::AdapterClientRequest {
                msg: Some(rpc::adapter_client_request::Msg::IpfsBlockGetRequest(
                    rpc::IpfsBlockGetRequest {
                        cid: cid.clone(),
                        wait_locally,
                        return_if_not_exist,
                        reply: format!("{}.{}", reply_to, uuid),
                    },
                )),
            })
        },
        callback,
    )
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

pub fn call_adapter_rpc(req: rpc::AdapterClientRequest) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                ref_seq: get_outbound_sequence()?,
                channel: ADAPTER_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::AdapterClientRequest(req)),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn call_adapter_rpc_async<F, R>(mut request_fun: R, callback: F) -> anyhow::Result<()>
where
    F: FnMut(&BrokerMessage) -> anyhow::Result<()> + Sync + Send + 'static,
    R: FnMut(&str) -> anyhow::Result<rpc::AdapterClientRequest> + Sync + Send + 'static,
{
    super::action::call_async(
        vmh_codec::VMH_CAPABILITY_ID,
        vmh_codec::OP_OUTBOUND_MESSAGE,
        move |uuid| {
            encode_protobuf(vmh::OutboundRequest {
                ref_seq: get_outbound_sequence()?,
                channel: ADAPTER_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::AdapterClientRequest(
                    request_fun(uuid)?,
                )),
            })
        },
        callback,
    )
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

pub fn register_adapter_dispatcher(type_ids: Vec<u32>) -> anyhow::Result<()> {
    untyped::default()
        .call(
            tea_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_REG_ADAPTER_DISPATCHER_MESSAGE,
            encode_protobuf(vmh::RegisterDispatcherRequest { type_ids })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}
