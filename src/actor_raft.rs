use prost::Message;
use std::collections::HashMap;
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::raft;
use wascc_actor::prelude::*;

pub fn raft_set_value(
    key: &str,
    value: &[u8],
    storage_index: u32,
    uuid: &str,
) -> anyhow::Result<()> {
    let response_vec = untyped::default()
        .call(
            tea_codec::RAFT_CAPABILITY_ID,
            "RaftSet",
            encode_protobuf(raft::SetValueRequest {
                key: key.to_string(),
                value: value.to_vec(),
                index: storage_index,
                uuid: uuid.to_string(),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let res = raft::SetValueResponse::decode(response_vec.as_slice())?;
    if !res.success {
        return Err(anyhow::anyhow!("raft set value failed: {:?}", res));
    }
    Ok(())
}

pub fn raft_get_value(key: &str, storage_index: u32, uuid: &str) -> anyhow::Result<Vec<u8>> {
    let response_vec = get_value(key, storage_index, uuid, false, false)?;
    let res = raft::GetValueResponse::decode(response_vec.as_slice())?;
    if let Some(v) = res.value {
        return Ok(v.value);
    }
    Err(anyhow::anyhow!("raft get value failed: {:?}", res))
}

pub fn raft_get_values(
    prefix: &str,
    storage_index: u32,
    uuid: &str,
) -> anyhow::Result<HashMap<String, Vec<u8>>> {
    let response_vec = get_value(prefix, storage_index, uuid, false, true)?;
    let res = raft::GetValueResponse::decode(response_vec.as_slice())?;
    if let Some(v) = res.values {
        let mut result = HashMap::new();
        for i in 0..v.keys.len() {
            result.insert(v.keys[i].clone(), v.values[i].clone());
        }
        return Ok(result);
    }
    Err(anyhow::anyhow!("raft get value failed: {:?}", res))
}

pub fn raft_remove_value(key: &str, storage_index: u32) -> anyhow::Result<()> {
    // todo implement me
    Ok(())
}

fn get_value(
    key: &str,
    storage_index: u32,
    uuid: &str,
    get_all: bool,
    get_by_prefix: bool,
) -> anyhow::Result<Vec<u8>> {
    untyped::default()
        .call(
            tea_codec::RAFT_CAPABILITY_ID,
            "RaftGet",
            encode_protobuf(raft::GetValueRequest {
                key: key.to_string(),
                index: storage_index,
                uuid: uuid.to_string(),
                get_all,
                get_by_prefix,
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))
}
