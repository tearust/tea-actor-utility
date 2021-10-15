use prost::Message;
use std::time::SystemTime;
use tea_codec::OP_CURRENT_TIMESTAMP;
use vmh_codec::message::encode_protobuf;
use vmh_codec::message::structs_proto::env;
use wascc_actor::prelude::*;

const CAPABILITY: &'static str = "tea:env";
const DEFAULT_DURATION: u128 = 10;

/// Return empty string is the env var is not set by the OS
pub fn get_env_var(env_var: &str) -> anyhow::Result<String> {
    let response_vec = untyped::default()
        .call(
            CAPABILITY,
            "GetEnvVar",
            encode_protobuf(env::GetRequest {
                key: env_var.to_string(),
            })?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let res = env::GetResponse::decode(response_vec.as_slice())?;
    if res.exists {
        Ok(res.value)
    } else {
        Err(anyhow::anyhow!(
            "failed to get environment variable: {}",
            env_var
        ))
    }
}

pub fn get_system_time() -> anyhow::Result<SystemTime> {
    let response_vec = untyped::default()
        .call(
            CAPABILITY,
            "GetSystemTime",
            encode_protobuf(env::GetSystemTimeRequest {})?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let res: SystemTime =
        deserialize(response_vec.as_slice()).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(res)
}

pub fn current_timestamp() -> anyhow::Result<i64> {
    let response_vec = untyped::default()
        .call(
            CAPABILITY,
            OP_CURRENT_TIMESTAMP,
            encode_protobuf(env::GetCurrentTimestampRequest {})?,
        )
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let res = env::GetCurrentTimestampResponse::decode(response_vec.as_slice())?;
    Ok(res.timestamp)
}

/// calculate elapsed time in milliseconds, if calculate duration failed returns default duration
/// instead, default duration is defined by `DEFAULT_DURATION` const.
pub fn time_since(earlier: SystemTime) -> anyhow::Result<u128> {
    let now = get_system_time()?;
    match now.duration_since(earlier) {
        Ok(d) => Ok(d.as_millis()),
        Err(e) => {
            warn!(
                "calculate duration failed: {}, returned default duration: {}",
                e, DEFAULT_DURATION
            );
            Ok(DEFAULT_DURATION)
        }
    }
}
