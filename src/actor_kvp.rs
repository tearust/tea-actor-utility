use tea_codec::{serialize, deserialize, keyvalue};
use wascc_actor::prelude::*;
use serde::{Serialize, Deserialize};
use keyvalue::*;

const CAPABILITY : &'static str = "tea:keyvalue";

pub fn set_forever<'de, T: Serialize + Deserialize<'de>> (binding_name: &'static str, key: &str, value: &T) -> HandlerResult<T> {
  let req = SetRequest{key: key.to_owned(), value:serialize(value)?, expires_s:0};
  let res : SetResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET,
    serialize(req)?
  )?.as_slice())?; 
  let result: T = deserialize(res.value.as_slice())?;
  Ok(result)
}

pub fn add (binding_name: &'static str, key: &str, value: i32) -> HandlerResult<i32> {
  let req = AddRequest{key: key.to_owned(), value:value};
  let res: AddResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_ADD,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.value)
}

pub fn del (binding_name: &'static str, key: &str) -> HandlerResult<String> {
  let req = DelRequest{key: key.to_owned()};
  let res: DelResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_DEL,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.key)
}

pub fn get<'de, T: Deserialize<'de>> (binding_name: &'static str, key: &str) -> HandlerResult<Option<T>> {
  let req = GetRequest{key: key.to_owned()};
  let res : GetResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_GET,
    serialize(req)?
  )?.as_slice())?;

  if res.exists {
    let result: T = deserialize(res.value.as_slice())?;
    Ok(Some(result))
  }
  else{
    Ok(None)
  }
}

pub fn list_clear(binding_name: &'static str, key: &str) -> HandlerResult<String> {
  let req = DelRequest{key: key.to_owned()};
  let res: DelResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_DEL,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.key)
}

pub fn list_range<'de, T: Serialize + Deserialize<'de>> (binding_name: &'static str, key: &str, start:i32, stop:i32) -> HandlerResult<Vec<T>> {
  let req = ListRangeRequest{key: key.to_owned(), start: start, stop: stop};
  let res: ListRangeResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_RANGE,
    serialize(req)?
  )?.as_slice())?;
  let result: Vec<T> = res.values.into_iter().map(|t| deserialize(t.as_slice()).unwrap()).collect();
  Ok(result)
}

pub fn list_push<'de, T: Serialize + Deserialize<'de>> (binding_name: &'static str, key: &str, value: &T) -> HandlerResult<i32> {
  let req = ListPushRequest{key: key.to_owned(), value: serialize(value)?};
  let res : ListResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_PUSH,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.new_count)
}

pub fn set<'de, T: Serialize + Deserialize<'de>> (binding_name: &'static str, key: &str, value: &T, expires_s:i32) -> HandlerResult<T> {
  let req = SetRequest{key: key.to_owned(), value: serialize(value)?, expires_s:expires_s};
  let res : SetResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET,
    serialize(req)?
  )?.as_slice())?;
  let result : T = deserialize(res.value.as_slice())?;
  Ok(result)
}


pub fn list_del_item<T: Serialize> (binding_name: &'static str, key: &str, value: &T) -> HandlerResult<i32> {
  let req = ListDelItemRequest{key: key.to_owned(), value: serialize(value)?};
  let res : ListResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_LIST_DEL,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.new_count)
}

pub fn set_add<T: Serialize> (binding_name: &'static str, key: &str, value: &T) ->HandlerResult<i32>{
  let req = SetAddRequest{key: key.to_owned(), value: serialize(value)?};
  let res: SetOperationResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET_ADD,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.new_count)
}


pub fn set_remove<T: Serialize> (binding_name: &'static str, key: &str, value: &T) ->HandlerResult<i32>{
  let req = SetRemoveRequest{key: key.to_owned(), value: serialize(value)?};
  let res: SetOperationResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET_REMOVE,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.new_count)
}




pub fn set_union<'de, T: Deserialize<'de>> (binding_name: &'static str, keys: Vec<&str>) -> HandlerResult<Vec<T>>{
  let keys: Vec<String> = keys.into_iter().map(|k| k.to_owned()).collect();
  let req = SetUnionRequest{keys: keys};
  let res: SetQueryResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET_UNION,
    serialize(req)?
  )?.as_slice())?;
  let result: Vec<T> = res.values.into_iter().map(|t| deserialize(t.as_slice()).unwrap()).collect();
  Ok(result)
}



pub fn set_intersect<'de, T: Deserialize<'de>> (binding_name: &'static str, keys: Vec<&str>) -> HandlerResult<Vec<T>>{
  let keys: Vec<String> = keys.into_iter().map(|k| k.to_owned()).collect();
  let req = SetIntersectionRequest{keys: keys};
  let res: SetQueryResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET_INTERSECT,
    serialize(req)?
  )?.as_slice())?;
  let result: Vec<T> = res.values.into_iter().map(|t| deserialize(t.as_slice()).unwrap()).collect();
  Ok(result)
}

pub fn set_query<'de, T: Deserialize<'de>> (binding_name: &'static str, key: &str) -> HandlerResult<Vec<T>>{
  let req = SetQueryRequest{key: key.to_owned()};
  let res: SetQueryResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_SET_QUERY,
    serialize(req)?
  )?.as_slice())?;
  let result: Vec<T> = res.values.into_iter().map(|t| deserialize(t.as_slice()).unwrap()).collect();
  Ok(result) 
}

pub fn exists(binding_name: &'static str, key: &str) -> HandlerResult<bool> {
  let req = KeyExistsQuery{key: key.to_owned()};
  let res : GetResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_KEY_EXISTS,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.exists)
}

pub fn keyvec_insert<T: Serialize>(binding_name: &'static str, key: &str, value:(i32, &T), overwrite: bool) -> HandlerResult<bool>{
  let req = KeyVecInsertQuery{key: key.to_string(), value: (value.0, serialize(value.1)?), overwrite: overwrite};
  let res: KeyVecInsertResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_KEYVEC_INSERT,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.success)
}

pub fn keyvec_get<'de, T: Deserialize<'de>> (binding_name: &'static str, key: &str) -> HandlerResult<Vec<(i32, T)>>{
  let req = KeyVecGetQuery{key: key.to_string()};
  let res: KeyVecGetResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_KEYVEC_GET,
    serialize(req)?
  )?.as_slice())?;

  let result: Vec<(i32, T)> = res.values.into_iter().map(|t| (t.0, deserialize(t.1.as_slice()).unwrap())).collect();
  Ok(result)

}

pub fn keyvec_tail_off(binding_name: &'static str, key: &str, remain: usize) -> HandlerResult<usize>{
  let req = KeyVecTailOffQuery{key: key.to_string(), remain: remain};
  let res: KeyVecTailOffResponse = deserialize(untyped::host(binding_name).call(
    CAPABILITY,
    OP_KEYVEC_TAILOFF,
    serialize(req)?
  )?.as_slice())?;
  Ok(res.len)
}

