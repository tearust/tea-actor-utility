use tea_codec::{serialize, deserialize, keyvalue};
use wascc_actor::prelude::*;

const CAPABILITY : &'static str = "tea:keyvalue";

pub fn set<'de, T: serde::Serialize + serde::Deserialize<'de>> (binding_name: &'static str, key: &str, value: T) -> HandlerResult<T>
{
  let req = keyvalue::SetRequest{key: key.to_owned(), value:serialize(value)?, expires_s:0};
  let result_bytes : keyvalue::SetResponse = tea_codec::deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    tea_codec::keyvalue::OP_SET,
    serialize(req)?
  )?.as_slice())?; 
  let result: T = deserialize(result_bytes.value.as_slice())?;
  Ok(result)
}

