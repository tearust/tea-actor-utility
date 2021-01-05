use super::env_proto;
use prost::Message;
use std::time::SystemTime;
use wascc_actor::prelude::*;
mod actor_ra_proto {
    include!(concat!(env!("OUT_DIR"), "/actor_ra.rs"));
}

const CAPABILITY: &'static str = "tea:env";

/// Return empty string is the env var is not set by the OS
pub fn get_env_var(env_var: &str) -> HandlerResult<String> {
    let req = env_proto::GetRequest {
        key: env_var.to_string(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("Cannot serilize req");
    let response_vec = untyped::default().call(CAPABILITY, "GetEnvVar", buf)?;

    let res = env_proto::GetResponse::decode(response_vec.as_slice())?;
    if res.exists {
        Ok(res.value)
    } else {
        Ok(String::new())
    }
}

pub fn get_system_time(param: &str) -> HandlerResult<SystemTime> {
    let req = env_proto::GetSystemTimeRequest {
        param: param.to_string(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("Cannot serilize req");
    let response_vec = untyped::default().call(CAPABILITY, "GetSystemTime", buf)?;

    let res: SystemTime = deserialize(response_vec.as_slice())?;
    Ok(res)
}

pub fn get_my_tea_id() -> HandlerResult<Vec<u8>> {
    let res_vec = untyped::default().call("tea:tpm", "GetTeaId", vec![])?;
    if res_vec.len() == 0 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "This is normal. You can ignore this error",
        )))
    } else {
        Ok(res_vec)
    }
}

pub fn get_my_ephemeral_id() -> HandlerResult<Vec<u8>> {
    let res_vec = untyped::default().call("tea:tpm", "GetEphemeralPubKey", vec![])?;

    if res_vec.len() == 0 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not init ephermeral key",
        )))
    } else {
        Ok(res_vec)
    }
}

pub fn get_my_ephemeral_sig() -> HandlerResult<Vec<u8>> {
    let res_vec = untyped::default().call("tea:tpm", "GetEphemeralSig", vec![])?;

    if res_vec.len() == 0 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not init ephermeral sig",
        )))
    } else {
        Ok(res_vec)
    }
}

pub fn get_my_signed_pcrs() -> HandlerResult<Vec<u8>> {
    let res = untyped::default().call("tea:tpm", "GetSignedPcrBytes", Vec::new())?;
    Ok(res)
}


