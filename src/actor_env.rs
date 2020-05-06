use tea_codec::{serialize, deserialize, keyvalue};
use wascc_actor::prelude::*;
use keyvalue::*;

const CAPABILITY : &'static str = "tea:keyvalue";
const BINDING_ENV : &'static str = "environment";
pub fn get_env(env_var: &str)->HandlerResult<String>{
  let req = GetRequest{key: String::from("__tea_env_") + env_var};
  let res : GetResponse = deserialize(untyped::host(BINDING_ENV).call(
    CAPABILITY,
    OP_GET,
    serialize(req)?
  )?.as_slice())?;
  if res.exists{
    let val: String = deserialize(res.value.as_slice())?;
    Ok(val)
  }
  else{
    Ok("".to_string())
  }
}