use crate::action;
use prost::Message;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{rpc, vmh},
};
use vmh_codec::ADAPTER_RPC_CHANNEL_NAME;
use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;

pub fn ipfs_info(peer_id: Option<String>, uuid: String) -> anyhow::Result<rpc::IpfsInfoResponse> {
    let rpc_res_bytes = call_adapter_rpc(
        rpc::AdapterClientRequest {
            msg: Some(rpc::adapter_client_request::Msg::IpfsInfoRequest(
                rpc::IpfsInfoRequest {
                    peer_id: peer_id.unwrap_or("".into()),
                },
            )),
        },
        uuid,
    )?;
    Ok(rpc::IpfsInfoResponse::decode(rpc_res_bytes.as_slice())?)
}

pub fn ipfs_block_get<F>(
    uuid: String,
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
        uuid,
    )
}

pub fn call_adapter_rpc(req: rpc::AdapterClientRequest, uuid: String) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                uuid,
                channel: ADAPTER_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::AdapterClientRequest(req)),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn call_adapter_rpc_async<F, R>(
    mut request_fun: R,
    callback: F,
    uuid: String,
) -> anyhow::Result<()>
where
    F: FnMut(&BrokerMessage) -> anyhow::Result<()> + Sync + Send + 'static,
    R: FnMut(&str) -> anyhow::Result<rpc::AdapterClientRequest> + Sync + Send + 'static,
{
    action::call_async(
        vmh_codec::VMH_CAPABILITY_ID,
        vmh_codec::OP_OUTBOUND_MESSAGE,
        move |callback_uuid| {
            encode_protobuf(vmh::OutboundRequest {
                uuid: uuid.clone(),
                channel: ADAPTER_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::AdapterClientRequest(
                    request_fun(callback_uuid)?,
                )),
            })
        },
        callback,
    )
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
