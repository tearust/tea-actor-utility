use tea_codec::{serialize, deserialize, env};
use std::time::SystemTime;
use wascc_actor::prelude::*;


const CAPABILITY : &'static str = "tea:env";
pub fn get_env_var(env_var: &str)->HandlerResult<(String, bool)>{
  let req = env::GetRequest{key: env_var.to_string()};
  let response_vec = untyped::default().call(
    CAPABILITY,
    env::OP_GET_VAR,
    serialize(req)?
  )?;

  let res : env::GetResponse = deserialize(response_vec.as_slice())?;
  Ok((res.value, res.exists))
}

pub fn get_system_time(param: &str) -> HandlerResult<SystemTime>{
  let req = env::GetSystemTimeRequest{param: param.to_string()};
  let response_vec = untyped::default().call(
    CAPABILITY,
    env::OP_GET_SYSTEM_TIME,
    serialize(req)?
  )?;

  let res : SystemTime = deserialize(response_vec.as_slice())?;
  Ok(res)
}