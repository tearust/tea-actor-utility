use std::time::SystemTime;
use wascc_actor::prelude::*;
use super::env_proto;
use prost::Message;

const CAPABILITY : &'static str = "tea:env";
pub fn get_env_var(env_var: &str)->HandlerResult<(String, bool)>{
  let req = env_proto::GetRequest{key: env_var.to_string()};
  let mut buf = Vec::with_capacity(req.encoded_len());
  req.encode(&mut buf).expect("Cannot serilize req");
  let response_vec = untyped::default().call(
    CAPABILITY,
    "GetEnvVar",
    buf
  )?;

  let res = env_proto::GetResponse::decode(response_vec.as_slice())?;
  Ok((res.value, res.exists))
}

pub fn get_system_time(param: &str) -> HandlerResult<SystemTime>{
  let req = env_proto::GetSystemTimeRequest{param: param.to_string()};
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

pub fn get_my_tea_id()-> HandlerResult<Vec<u8>>{
  info!("In local test mode, use fixed df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b as tea_id");
  Ok(hex::decode("df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b")?)
}

pub fn get_my_tea_privkey()-> HandlerResult<Vec<u8>>{
  info!("In local test mode, use placeholder private key fixed 5579a3c220146f0caaab49b884de505098b89326970b929d781cf4a65445a917df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b as tea_id");
  Ok(hex::decode("5579a3c220146f0caaab49b884de505098b89326970b929d781cf4a65445a917df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b")?)

}