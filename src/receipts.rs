use crate::actor_ipfs::ipfs_block_put;
use prost::Message;
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::{env, kvp, receipt, vmh};
use wascc_actor::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PriceParams {
    pub unit_price: u64,
    pub price_coefficient: u64,
}

pub fn start_task(uuid: &str) -> anyhow::Result<()> {
    untyped::default()
        .call(
            vmh_codec::ENV_CAPABILITY_ID,
            "TaskStart",
            encode_protobuf(env::StartTasksRequest {
                uuid: uuid.to_string(),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

pub fn end_task(
    uuid: &str,
    inbound_net_params: PriceParams,
    outbound_net_params: PriceParams,
    timespan_params: PriceParams,
) -> anyhow::Result<String> {
    let end_task_res = env::EndTasksResponse::decode(
        untyped::default()
            .call(
                vmh_codec::ENV_CAPABILITY_ID,
                "TaskEnd",
                encode_protobuf(env::EndTasksRequest {
                    uuid: uuid.to_string(),
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;

    let inbound_net_res = vmh::NetworkDataLenResponse::decode(
        untyped::default()
            .call(
                vmh_codec::VMH_CAPABILITY_ID,
                vmh_codec::OP_INBOUND_NETWORK_DATA_LEN,
                encode_protobuf(vmh::NetworkDataLenRequest {
                    uuid: uuid.to_string(),
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    if !inbound_net_res.error.is_empty() {
        return Err(anyhow::anyhow!("{}", inbound_net_res.error));
    }

    let outbound_net_res = vmh::NetworkDataLenResponse::decode(
        untyped::default()
            .call(
                vmh_codec::VMH_CAPABILITY_ID,
                vmh_codec::OP_OUTBOUND_NETWORK_DATA_LEN,
                encode_protobuf(vmh::NetworkDataLenRequest {
                    uuid: uuid.to_string(),
                })?,
            )
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .as_slice(),
    )?;
    if !outbound_net_res.error.is_empty() {
        return Err(anyhow::anyhow!("{}", outbound_net_res.error));
    }

    let task_receipt = receipt::TaskReceipt {
        uuid: uuid.to_string(),
        network: Some(receipt::Network {
            inbound: Some(receipt::Inbound {
                bytes: inbound_net_res.len,
                unit_price: inbound_net_params.unit_price,
                price_coefficient: inbound_net_params.price_coefficient,
            }),
            outbound: Some(receipt::Outbound {
                bytes: outbound_net_res.len,
                unit_price: outbound_net_params.unit_price,
                price_coefficient: outbound_net_params.price_coefficient,
            }),
        }),
        timespan: Some(receipt::Timespan {
            milliseconds: end_task_res.milliseconds,
            unit_price: timespan_params.unit_price,
            price_coefficient: timespan_params.price_coefficient,
        }),
    };
    let (cid, _) = ipfs_block_put(encode_protobuf(task_receipt)?.as_slice(), true, uuid)?;
    Ok(cid)
}

/// get memory and disk usage receipt, `duration` is an u128 number with unit of millisecond
pub fn get_storage_receipts(
    actors: &Vec<String>,
    uuid: &str,
    duration: u128,
    memory_params: PriceParams,
    disk_params: PriceParams,
) -> anyhow::Result<String> {
    let mut total_memory_size = 0;
    for actor in actors {
        let res = kvp::TaskMemorySizeResponse::decode(
            untyped::host(actor)
                .call(
                    vmh_codec::KVP_CAPABILITY_ID,
                    "GetTaskMemSize",
                    encode_protobuf(kvp::TaskMemorySizeRequest {
                        uuid: uuid.to_string(),
                    })?,
                )
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .as_slice(),
        )?;
        total_memory_size += res.size;
    }

    let storage_receipt = receipt::StorageReceipt {
        uuid: uuid.to_string(),
        memory: Some(receipt::Memory {
            bytes: total_memory_size,
            duration: serialize(duration).map_err(|e| anyhow::anyhow!("{}", e))?,
            unit_price: memory_params.unit_price,
            price_coefficient: memory_params.price_coefficient,
        }),
        disk: Some(receipt::Disk {
            bytes: 0, // todo calculate later from IPFS
            duration: serialize(duration).map_err(|e| anyhow::anyhow!("{}", e))?,
            unit_price: disk_params.unit_price,
            price_coefficient: disk_params.price_coefficient,
        }),
    };

    let (cid, _) = ipfs_block_put(encode_protobuf(storage_receipt)?.as_slice(), true, uuid)?;
    Ok(cid)
}
