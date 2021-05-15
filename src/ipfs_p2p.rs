use serde::{Deserialize, Serialize};
use vmh_codec::message::structs_proto::p2p::GeneralMsg;
use vmh_codec::message::{
    encode_protobuf,
    structs_proto::{p2p, rpc},
};

const PREFIX_P2P_REPLY: &str = "ipfs.p2p.reply";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2pReplyType {
    Success,
    Cancelled,
    Rejected,
    Error(String),
}

pub fn send_message(peer_id: &str, uuid: &str, msg: GeneralMsg) -> anyhow::Result<()> {
    let peer_id = peer_id.to_string();
    let reply = format!("{}.{}", PREFIX_P2P_REPLY, uuid);
    let _ = super::actor_rpc::call_adapter_rpc(
        rpc::AdapterClientRequest {
            msg: Some(rpc::adapter_client_request::Msg::IpfsP2pFrowardRequest(
                rpc::IpfsP2pFrowardRequest {
                    peer_id,
                    reply,
                    p2p_general_msg: Some(msg),
                },
            )),
        },
        uuid.to_string(),
    )?;

    Ok(())
}

pub fn async_pull_cid_data(
    uuid: &str,
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
    let _ = super::actor_rpc::call_adapter_rpc(
        rpc::AdapterClientRequest {
            msg: Some(rpc::adapter_client_request::Msg::IpfsPullCidDataRequest(
                rpc::IpfsPullCidDataRequest {
                    peer_id,
                    reply,
                    payload,
                    pin,
                    cid,
                },
            )),
        },
        uuid.to_string(),
    )?;

    Ok(())
}

pub fn close_p2p(peer_id: &str, uuid: String) -> anyhow::Result<()> {
    let peer_id = peer_id.to_string();
    let _ = super::actor_rpc::call_adapter_rpc(
        rpc::AdapterClientRequest {
            msg: Some(rpc::adapter_client_request::Msg::IpfsP2pCloseRequest(
                rpc::IpfsP2pCloseRequest { peer_id },
            )),
        },
        uuid,
    )?;

    Ok(())
}

pub fn response_ipfs_p2p_with_error(
    peer_id: &str,
    uuid: &str,
    error: &str,
) -> anyhow::Result<Vec<u8>> {
    info!(
        "ipfs_p2p.rs peer_id:{}, uuid:{}, error:{}",
        peer_id, uuid, error
    );
    error!("response_ipfs_p2p_with_error: {}", error);
    let msg = p2p::P2pReplyMessage {
        peer_id: peer_id.to_string(),
        uuid: uuid.to_string(),
        content: "".to_string(),
        reply_type: p2p::P2pReplyType::Error as i32,
        reply_error: Some(p2p::P2pReplyError {
            message: error.to_string(),
        }),
    };
    response_ipfs_p2p_reply_with_subject(msg)
}

pub fn response_ipfs_p2p(
    peer_id: &str,
    uuid: &str,
    content: &str,
    ty: P2pReplyType,
) -> anyhow::Result<Vec<u8>> {
    let mut msg = p2p::P2pReplyMessage {
        peer_id: peer_id.to_string(),
        uuid: uuid.to_string(),
        content: content.to_string(),
        reply_type: p2p::P2pReplyType::Success as i32,
        reply_error: None,
    };
    set_reply_message_type(&mut msg, ty);
    response_ipfs_p2p_reply_with_subject(msg)
}

fn set_reply_message_type(msg: &mut p2p::P2pReplyMessage, ty: P2pReplyType) {
    match ty {
        P2pReplyType::Success => msg.set_reply_type(p2p::P2pReplyType::Success),
        P2pReplyType::Cancelled => msg.set_reply_type(p2p::P2pReplyType::Cancelled),
        P2pReplyType::Rejected => msg.set_reply_type(p2p::P2pReplyType::Rejected),
        P2pReplyType::Error(message) => {
            msg.set_reply_type(p2p::P2pReplyType::Error);
            msg.reply_error = Some(p2p::P2pReplyError { message })
        }
    }
}

pub fn response_ipfs_p2p_reply_with_subject(
    p2p_reply_message: p2p::P2pReplyMessage,
) -> anyhow::Result<Vec<u8>> {
    encode_protobuf(rpc::IpfsInboundP2pForwardResponse {
        p2p_reply_message: Some(p2p_reply_message),
    })
}

pub fn log_and_response(
    peer_id: &str,
    uuid: &str,
    content: &str,
    ty: P2pReplyType,
) -> anyhow::Result<Vec<u8>> {
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
    response_ipfs_p2p(peer_id, uuid, content, ty)
}

pub fn log_and_response_with_error(
    peer_id: &str,
    uuid: &str,
    error: &str,
) -> anyhow::Result<Vec<u8>> {
    error!("{}", error);
    response_ipfs_p2p_with_error(peer_id, uuid, error)
}
