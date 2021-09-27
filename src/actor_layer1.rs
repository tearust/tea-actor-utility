use prost::Message;
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::layer1;
use wascc_actor::prelude::*;

pub fn register_layer1_event() -> anyhow::Result<()> {
    untyped::default()
        .call(
            tea_codec::LAYER1_CAPABILITY_ID,
            vmh_codec::OP_REG_LAYER1_EVENT_MESSAGE,
            encode_protobuf(layer1::RegisterLayer1EventRequest {})?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

pub fn general_remote_request(req: layer1::Layer1Outbound) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            tea_codec::LAYER1_CAPABILITY_ID,
            "GeneralRequest",
            encode_protobuf(req)?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn transfer_balance(
    source_seed: Vec<u8>,
    to_address: &str,
    amount: u128,
) -> anyhow::Result<()> {
    let api_info_res = layer1_api_info()?;
    let to_pub_key = crate::actor_crypto::public_key_from_ss58(to_address)?;
    let construct_tx_res = construct_transfer_tx(source_seed, to_pub_key, amount, api_info_res)?;
    let send_tx_res = send_tx(construct_tx_res.raw_transaction)?;
    info!("transfer balance got balance: {:?}", send_tx_res);
    Ok(())
}

pub fn layer1_api_info() -> anyhow::Result<layer1::ApiInfoResponse> {
    Ok(layer1::ApiInfoResponse::decode(
        untyped::default()
            .call(
                tea_codec::LAYER1_CAPABILITY_ID,
                "Layer1ApiInfo",
                encode_protobuf(layer1::ApiInfoRequest {})?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?)
}

pub fn construct_transfer_tx(
    source_seed: Vec<u8>,
    to_pub_key: Vec<u8>,
    amount: u128,
    api_info: layer1::ApiInfoResponse,
) -> anyhow::Result<layer1::ConstructExtrinsicResponse> {
    Ok(layer1::ConstructExtrinsicResponse::decode(
        untyped::default()
            .call(
                tea_codec::LAYER1_CAPABILITY_ID,
                "ConstructTx",
                encode_protobuf(layer1::ConstructExtrinsicRequest {
                    to_public_key: to_pub_key,
                    amount: amount.to_le_bytes().to_vec(),
                    private_key: source_seed,
                    substrate_api_info: api_info.substrate_api_info,
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?)
}

pub fn send_tx(raw_transaction: Vec<u8>) -> anyhow::Result<layer1::SendTxResponse> {
    Ok(layer1::SendTxResponse::decode(
        untyped::default()
            .call(
                tea_codec::LAYER1_CAPABILITY_ID,
                "SendTx",
                encode_protobuf(layer1::SendTxRequest { raw_transaction })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?)
}
