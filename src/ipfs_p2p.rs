use crate::actor_nats::response_reply_with_subject;
use codec::messaging::BrokerMessage;
use prost::Message;
use serde::{Deserialize, Serialize};
use tea_codec::error::TeaError;
use vmh_codec::message::structs_proto::p2p::GeneralMsg;
use vmh_codec::message::{encode_protobuf, structs_proto::rpc};
use wascc_actor::prelude::*;

const PREFIX_P2P_REPLY: &str = "ipfs.p2p.reply";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2pReplyType {
    Success,
    Cancelled,
    Rejected,
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2pReplyMessage {
    pub uuid: String,
    pub peer_id: String,
    pub op_type: P2pReplyType,
    pub content: String,
}

pub fn listen_message<F>(from_peer_id: &str, msg: &BrokerMessage, callback: F) -> anyhow::Result<()>
where
    F: Fn(&GeneralMsg, &str, &str) -> HandlerResult<()> + Sync + Send + 'static,
{
    match GeneralMsg::decode(msg.body.as_slice()) {
        Ok(ori_msg) => {
            if let Err(err) = callback(&ori_msg, from_peer_id, &msg.reply_to) {
                return response_ipfs_p2p_with_error(
                    &msg.reply_to,
                    from_peer_id,
                    "",
                    &format!("[listen_message] execute callback error: {:?}", err),
                );
            }
            Ok(())
        }
        Err(err) => {
            error!(
                "GeneralMsg::decode error. body is {:?}, error is {:?}",
                &msg.body, &err
            );
            response_ipfs_p2p_with_error(
                &msg.reply_to,
                from_peer_id,
                "",
                &format!("[listen_message] decode GeneralMsg error: {:?}", err),
            )
        }
    }
}

pub fn send_message(peer_id: &str, uuid: &str, msg: GeneralMsg) -> anyhow::Result<()> {
    let peer_id = peer_id.to_string();
    let reply = format!("{}.{}", PREFIX_P2P_REPLY, uuid);
    let payload = encode_protobuf(msg).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let _ = super::actor_rpc::call_adapter_rpc(rpc::AdapterClientRequest {
        msg: Some(rpc::adapter_client_request::Msg::IpfsP2pFrowardRequest(
            rpc::IpfsP2pFrowardRequest {
                peer_id,
                reply,
                payload,
            },
        )),
    })?;

    Ok(())
}

pub fn async_pull_cid_data(
    peer_id: &str,
    cid: &str,
    payload: &[u8],
    pin: bool,
    reply_to: &str,
) -> anyhow::Result<()> {
    let peer_id = peer_id.to_string();
    let payload = payload.to_vec();
    let reply = reply_to.to_string();
    let cid = cid.to_string();
    let _ = super::actor_rpc::call_adapter_rpc(rpc::AdapterClientRequest {
        msg: Some(rpc::adapter_client_request::Msg::IpfsPullCidDataRequest(
            rpc::IpfsPullCidDataRequest {
                peer_id,
                reply,
                payload,
                pin,
                cid,
            },
        )),
    })?;

    Ok(())
}

pub fn close_p2p(peer_id: &str) -> HandlerResult<()> {
    let peer_id = peer_id.to_string();
    let _ = super::actor_rpc::call_adapter_rpc(rpc::AdapterClientRequest {
        msg: Some(rpc::adapter_client_request::Msg::IpfsP2pCloseRequest(
            rpc::IpfsP2pCloseRequest { peer_id },
        )),
    })?;

    Ok(())
}

pub fn response_ipfs_p2p_with_error(
    subject: &str,
    peer_id: &str,
    uuid: &str,
    error: &str,
) -> anyhow::Result<()> {
    info!(
        "ipfs_p2p.rs subject:{}, peer_id:{}, uuid:{}, error:{}",
        subject, peer_id, uuid, error
    );
    error!("response_ipfs_p2p_with_error: {}", error);
    response_ipfs_p2p_reply_with_subject(
        "",
        subject,
        &P2pReplyMessage {
            peer_id: peer_id.to_string(),
            uuid: uuid.to_string(),
            op_type: P2pReplyType::Error(error.to_string()),
            content: "".to_string(),
        },
    )
}

pub fn response_ipfs_p2p(
    subject: &str,
    peer_id: &str,
    uuid: &str,
    content: &str,
    ty: P2pReplyType,
) -> anyhow::Result<()> {
    response_ipfs_p2p_reply_with_subject(
        "",
        subject,
        &P2pReplyMessage {
            peer_id: peer_id.to_string(),
            uuid: uuid.to_string(),
            op_type: ty,
            content: content.to_string(),
        },
    )
}

pub fn response_ipfs_p2p_reply_with_subject(
    reply_to: &str,
    subject: &str,
    msg: &P2pReplyMessage,
) -> anyhow::Result<()> {
    let body = serialize(msg).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    response_reply_with_subject(reply_to, subject, body)
}

pub fn log_and_response(
    subject: &str,
    peer_id: &str,
    uuid: &str,
    content: &str,
    ty: P2pReplyType,
) -> HandlerResult<()> {
    match &ty {
        P2pReplyType::Success => {
            // if success do not print log locally
            // info!(
            //     "Success, content: {}, peer_id: {}, uuid: {}, reply type: {:?}",
            //     content, peer_id, uuid, ty
            // )
        }
        P2pReplyType::Error(err) => error!(
            "Content: {}, peer_id: {}, uuid: {}, error details: {}",
            content, peer_id, uuid, err
        ),
        _ => warn!(
            "Content: {}, peer_id: {}, uuid: {}, reply type: {:?}",
            content, peer_id, uuid, ty
        ),
    }
    Ok(response_ipfs_p2p(subject, peer_id, uuid, content, ty)?)
}

pub fn log_and_response_with_error(
    subject: &str,
    peer_id: &str,
    uuid: &str,
    error: &str,
) -> anyhow::Result<()> {
    error!("{}", error);
    Ok(response_ipfs_p2p_with_error(subject, peer_id, uuid, error)?)
}
