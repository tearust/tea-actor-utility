use tea_codec::{serialize, deserialize, keyvalue};
use wascc_actor::prelude::*;


pub fn set<'de, T: serde::Serialize + serde::Deserialize<'de>> (key: String, value: T) -> HandlerResult<T>
{
  let source1 = untyped::host("bootstrap_kvp"); 
  let req = keyvalue::SetRequest{key, value:serialize(value)?, expires_s:0};
  let result_bytes : keyvalue::SetResponse = tea_codec::deserialize(source1.call(
    "tea:keyvalue",
    "Set",
    serialize(req)?
  )?.as_slice())?; 
  let result: T = deserialize(result_bytes.value.as_slice())?;
  Ok(result)
}

