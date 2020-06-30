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
  info!("In local test mode, use fixed e9889b1c54ccd6cf184901ded892069921d76f7749b6f73bed6cf3b9be1a8a44 as tea_id");
  Ok(hex::decode("e9889b1c54ccd6cf184901ded892069921d76f7749b6f73bed6cf3b9be1a8a44")?)
}

pub fn get_my_tea_privkey()-> HandlerResult<Vec<u8>>{
  info!("In local test mode, use fixed 00f86ad55a93d71927a87b36e11b6845e4999160aa81eb5758a5b3872bd72f01 as tea_id");
  Ok(hex::decode("00f86ad55a93d71927a87b36e11b6845e4999160aa81eb5758a5b3872bd72f01")?)

}