use crate::actor_nats::response_reply_with_subject;
use crate::p2p_proto::GeneralMsg;
use crate::{actor_ra_proto, encode_protobuf};
use codec::messaging;
use codec::messaging::BrokerMessage;
use prost::Message;
use serde::{Deserialize, Serialize};
use tea_codec::error::TeaError;
use wascc_actor::prelude::*;

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
    let reply = &msg.clone().reply_to;
    match serialize(BrokerMessage {
        subject: reply.to_string(),
        reply_to: String::new(),
        body: "received".as_bytes().to_vec(),
    }) {
        Ok(payload) => {
            if let Err(err) =
                untyped::default().call("wascc:messaging", messaging::OP_PUBLISH_MESSAGE, payload)
            {
                return response_ipfs_p2p_with_error(
                    &msg.reply_to,
                    from_peer_id,
                    "",
                    &format!("[listen_message] send message error: {:?}", err),
                );
            }

            match base64::decode(msg.body.clone()) {
                Ok(ori_msg_bytes) => match GeneralMsg::decode(ori_msg_bytes.as_slice()) {
                    Ok(ori_msg) => {
                        if let Err(err) = callback(&ori_msg, from_peer_id, &msg.reply_to) {
                            return response_ipfs_p2p_with_error(
                                &msg.reply_to,
                                from_peer_id,
                                "",
                                &format!("[listen_message] execute callback error: {:?}", err),
                            );
                        }
                    }
                    Err(err) => response_ipfs_p2p_with_error(
                        &msg.reply_to,
                        from_peer_id,
                        "",
                        &format!("[listen_message] decode GeneralMsg error: {:?}", err),
                    )?,
                },
                Err(err) => response_ipfs_p2p_with_error(
                    &msg.reply_to,
                    from_peer_id,
                    "",
                    &format!("[listen_message] decode raw message error: {:?}", err),
                )?,
            }
        }
        Err(err) => {
            response_ipfs_p2p_with_error(
                &msg.reply_to,
                from_peer_id,
                "",
                &format!("[listen_message] serialize BrokerMessage error: {:?}", err),
            )?;
        }
    }

    Ok(())
}

pub fn send_message(peer_id: &str, uuid: &str, msg: GeneralMsg) -> anyhow::Result<()> {
    let nats_key = format!("ipfs.p2p.forward.{}", peer_id);
    let msg_bytes =
        crate::encode_protobuf(msg).map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    let msg_b64 = base64::encode(msg_bytes);
    if let Err(e) = untyped::default().call(
        "wascc:messaging",
        messaging::OP_PUBLISH_MESSAGE,
        serialize(BrokerMessage {
            subject: nats_key.to_string(),
            reply_to: format!("ipfs.p2p.reply.{}", uuid),
            body: msg_b64.as_bytes().to_vec(),
        })
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?,
    ) {
        error!("p2p send message with error {}", e);
    }

    Ok(())
}

pub fn async_pull_cid_data(
    peer_id: &str,
    cid: &str,
    payload: &[u8],
    pin: bool,
    reply_to: &str,
) -> anyhow::Result<()> {
    let nats_key = format!("ipfs.p2p.pull_cid_data.{}", peer_id);
    info!("async_pull_cid_data reply_to: {}", &reply_to);
    let req = actor_ra_proto::AsyncPullCidDataRequest {
        cid: cid.to_string(),
        payload: payload.to_vec(),
        pin,
    };
    if let Err(e) = untyped::default().call(
        "wascc:messaging",
        messaging::OP_PUBLISH_MESSAGE,
        serialize(BrokerMessage {
            subject: nats_key.to_string(),
            reply_to: reply_to.to_string(),
            body: encode_protobuf(req).map_err(|e| TeaError::CommonError(format!("{}", e)))?,
        })
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?,
    ) {
        error!("pull cid data with error {}", e);
    }

    Ok(())
}

pub fn close_p2p(peer_id: &str) -> HandlerResult<()> {
    let close_sub = format!("ipfs.p2p.close.{}", peer_id);
    info!("channel closed with => {}", close_sub);
    untyped::default().call(
        "wascc:messaging",
        messaging::OP_PUBLISH_MESSAGE,
        serialize(BrokerMessage {
            subject: close_sub.to_string(),
            reply_to: String::new(),
            body: "".as_bytes().to_vec(),
        })?,
    )?;

    Ok(())
}

pub fn response_ipfs_p2p_with_error(
    subject: &str,
    peer_id: &str,
    uuid: &str,
    error: &str,
) -> anyhow::Result<()> {
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
