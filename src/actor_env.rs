use std::time::SystemTime;
use wascc_actor::prelude::*;
use super::structs_proto;
use prost::Message;

const CAPABILITY : &'static str = "tea:env";
pub fn get_env_var(env_var: &str)->HandlerResult<(String, bool)>{
  let req = structs_proto::GetRequest{key: env_var.to_string()};
  let mut buf = Vec::with_capacity(req.encoded_len());
  req.encode(&mut buf).expect("Cannot serilize req");
  let response_vec = untyped::default().call(
    CAPABILITY,
    "GetEnvVar",
    buf
  )?;

  let res = structs_proto::GetResponse::decode(response_vec.as_slice())?;
  Ok((res.value, res.exists))
}

pub fn get_system_time(param: &str) -> HandlerResult<SystemTime>{
  let req = structs_proto::GetSystemTimeRequest{param: param.to_string()};
  let mut buf = Vec::with_capacity(req.encoded_len());
  req.encode(&mut buf).expect("Cannot serilize req");
  let response_vec = untyped::default().call(
    CAPABILITY,
    "GetSystemTime",
    buf
  )?;

  let res : SystemTime = deserialize(response_vec.as_slice())?;
  Ok(res)
}