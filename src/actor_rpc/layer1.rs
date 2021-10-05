use prost::Message;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{rpc, vmh},
};
use vmh_codec::LAYER1_RPC_CHANNEL_NAME;
use wascc_actor::prelude::*;

pub fn execute_http_request(
    req_url: &str,
    uuid: String,
    headers: Vec<rpc::HttpExecutionHeader>,
) -> anyhow::Result<String> {
    let rpc_res_bytes = call_layer1_rpc(
        rpc::Layer1GeneralRequest {
            msg: Some(rpc::layer1_general_request::Msg::HttpExecutionRequest(
                rpc::HttpExecutionRequest {
                    request_url: req_url.to_string(),
                    headers,
                },
            )),
        },
        uuid,
    )?;
    let res = rpc::Layer1GeneralResponse::decode(rpc_res_bytes.as_slice())?;
    if let Some(rpc::layer1_general_response::Msg::HttpExecutionResponse(res)) = res.msg {
        return Ok(res.response_json);
    }
    Err(anyhow::anyhow!("unknown response: {:?}", res))
}

pub fn call_layer1_rpc(req: rpc::Layer1GeneralRequest, uuid: String) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            vmh_codec::VMH_CAPABILITY_ID,
            vmh_codec::OP_OUTBOUND_MESSAGE,
            encode_protobuf(vmh::OutboundRequest {
                uuid,
                channel: LAYER1_RPC_CHANNEL_NAME.into(),
                msg: Some(vmh::outbound_request::Msg::Layer1GeneralRequest(req)),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}
