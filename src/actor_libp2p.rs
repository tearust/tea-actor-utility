use tea_codec::LIBP2P_CAPABILITY_ID;
use vmh_codec::message::{encode_protobuf, structs_proto::libp2p};
use wascc_actor::untyped;

pub fn my_conn_id() -> anyhow::Result<String> {
    let conn_id = untyped::default()
        .call(
            LIBP2P_CAPABILITY_ID,
            "MyConnId",
            encode_protobuf(libp2p::MyConnIdRequest {})?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(String::from_utf8(conn_id)?)
}

pub fn send_message(
    target_conn_id: String,
    target_address: libp2p::RuntimeAddress,
    source_address: Option<libp2p::RuntimeAddress>,
    content: Vec<u8>,
) -> anyhow::Result<()> {
    untyped::default()
        .call(
            LIBP2P_CAPABILITY_ID,
            "SendMessage",
            encode_protobuf(libp2p::GeneralRequest {
                source_conn_id: Default::default(),
                target_conn_id,
                runtime_message: Some(libp2p::RuntimeMessage {
                    source_address,
                    target_address: Some(target_address),
                    content,
                }),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

pub fn pub_message(
    target_address: libp2p::RuntimeAddress,
    source_address: Option<libp2p::RuntimeAddress>,
    content: Vec<u8>,
) -> anyhow::Result<()> {
    untyped::default()
        .call(
            LIBP2P_CAPABILITY_ID,
            "PubMessage",
            encode_protobuf(libp2p::PubMessage {
                source_conn_id: Default::default(),
                runtime_message: Some(libp2p::RuntimeMessage {
                    source_address,
                    target_address: Some(target_address),
                    content,
                }),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}
