use prost::Message;
use serde::{Deserialize, Serialize};
use tea_codec::error::TeaError;
use tea_codec::{deserialize, serialize};
use vmh_codec::message::structs_proto::kvp;
use wascc_actor::prelude::*;

const CAPABILITY: &'static str = "tea:keyvalue";

pub struct ShabbyLock {
    key: String,
    binding_name: &'static str,
}

impl ShabbyLock {
    pub fn lock(binding_name: &'static str, uid: &str) -> Self {
        let key = format!("ShabbyLock_{}", uid);
        trace!("enter lock {}", &key);
        loop {
            let t: anyhow::Result<Option<String>> = get(binding_name, &key);
            match t {
                Ok(res) => match res {
                    Some(_) => {
                        trace!("ShabbyLock is waiting for lock...in loop...");
                        continue;
                    }
                    None => {
                        let _ = set(binding_name, &key, b"lock", 6000);
                        trace!("Received lock");
                        break;
                    }
                },
                Err(_) => continue,
            }
        }
        ShabbyLock { key, binding_name }
    }
}

impl Drop for ShabbyLock {
    fn drop(&mut self) {
        trace!("drop ShabbyLock");
        let _ = del(&self.binding_name, &self.key);
    }
}

pub fn set_forever<'de, T: Serialize + Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
    value: &T,
) -> anyhow::Result<T> {
    let req = kvp::SetRequest {
        key: key.to_owned(),
        value: serialize(value)?,
        expires_s: 0,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Set", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    let result: T = deserialize(res.value.as_slice())?;
    Ok(result)
}

pub fn add(binding_name: &'static str, key: &str, value: i32) -> HandlerResult<i32> {
    let req = kvp::AddRequest {
        key: key.to_owned(),
        value: value,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::AddResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Add", buf)?
            .as_slice(),
    )?;
    Ok(res.value)
}

pub fn del(binding_name: &'static str, key: &str) -> HandlerResult<String> {
    let req = kvp::DelRequest {
        key: key.to_owned(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::DelResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Del", buf)?
            .as_slice(),
    )?;
    Ok(res.key)
}

pub fn get<'de, T: Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
) -> anyhow::Result<Option<T>> {
    let req = kvp::GetRequest {
        key: key.to_owned(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::GetResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Get", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;

    if res.exists {
        match res.value {
            Some(kvp::get_response::Value::V(value)) => {
                let result: T = deserialize(&value)?;
                Ok(Some(result))
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn list_clear(binding_name: &'static str, key: &str) -> HandlerResult<String> {
    let req = kvp::DelRequest {
        key: key.to_owned(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::DelResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Del", buf)?
            .as_slice(),
    )?;
    Ok(res.key)
}

pub fn list_range<'de, T: Serialize + Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
    start: i32,
    stop: i32,
) -> HandlerResult<Vec<T>> {
    let req = kvp::ListRangeRequest {
        key: key.to_owned(),
        start: start,
        stop: stop,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::ListRangeResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Range", buf)?
            .as_slice(),
    )?;
    let result: Vec<T> = res
        .values
        .into_iter()
        .map(|t| deserialize(t.as_slice()).unwrap())
        .collect();
    Ok(result)
}

pub fn list_push<'de, T: Serialize + Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
    value: &T,
) -> anyhow::Result<i32> {
    let req = kvp::ListPushRequest {
        key: key.to_owned(),
        value: serialize(value)?,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::ListResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Push", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    Ok(res.new_count)
}

pub fn set<'de, T: Serialize + Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
    value: &T,
    expires_s: i32,
) -> anyhow::Result<T> {
    let req = kvp::SetRequest {
        key: key.to_owned(),
        value: serialize(value)?,
        expires_s: expires_s,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "Set", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    let result: T = deserialize(res.value.as_slice())?;
    Ok(result)
}

pub fn list_del_item<T: Serialize>(
    binding_name: &'static str,
    key: &str,
    value: &T,
) -> anyhow::Result<i32> {
    let req = kvp::ListDelItemRequest {
        key: key.to_owned(),
        value: serialize(value)?,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::ListResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "ListItemDelete", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    Ok(res.new_count)
}

pub fn set_add<T: Serialize>(
    binding_name: &'static str,
    key: &str,
    value: &T,
) -> anyhow::Result<i32> {
    let req = kvp::SetAddRequest {
        key: key.to_owned(),
        value: serialize(value)?,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetOperationResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "SetAdd", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    Ok(res.new_count)
}

pub fn set_remove<T: Serialize>(
    binding_name: &'static str,
    key: &str,
    value: &T,
) -> anyhow::Result<i32> {
    let req = kvp::SetRemoveRequest {
        key: key.to_owned(),
        value: serialize(value)?,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetOperationResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "SetRemove", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    Ok(res.new_count)
}

pub fn set_union<'de, T: Deserialize<'de>>(
    binding_name: &'static str,
    keys: Vec<&str>,
) -> HandlerResult<Vec<T>> {
    let keys: Vec<String> = keys.into_iter().map(|k| k.to_owned()).collect();
    let req = kvp::SetUnionRequest { keys: keys };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetQueryResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "SetUnion", buf)?
            .as_slice(),
    )?;
    let result: Vec<T> = res
        .values
        .into_iter()
        .map(|t| deserialize(t.as_slice()).unwrap())
        .collect();
    Ok(result)
}

pub fn set_intersect<'de, T: Deserialize<'de>>(
    binding_name: &'static str,
    keys: Vec<&str>,
) -> HandlerResult<Vec<T>> {
    let keys: Vec<String> = keys.into_iter().map(|k| k.to_owned()).collect();
    let req = kvp::SetIntersectionRequest { keys: keys };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetQueryResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "SetIntersection", buf)?
            .as_slice(),
    )?;
    let result: Vec<T> = res
        .values
        .into_iter()
        .map(|t| deserialize(t.as_slice()).unwrap())
        .collect();
    Ok(result)
}

pub fn set_query<'de, T: Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
) -> HandlerResult<Vec<T>> {
    let req = kvp::SetQueryRequest {
        key: key.to_owned(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::SetQueryResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "SetQuery", buf)?
            .as_slice(),
    )?;
    let result: Vec<T> = res
        .values
        .into_iter()
        .map(|t| deserialize(t.as_slice()).unwrap())
        .collect();
    Ok(result)
}

pub fn exists(binding_name: &'static str, key: &str) -> HandlerResult<bool> {
    let req = kvp::KeyExistsQuery {
        key: key.to_owned(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::GetResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "KeyExists", buf)?
            .as_slice(),
    )?;
    Ok(res.exists)
}

pub fn keyvec_insert<T: Serialize>(
    binding_name: &'static str,
    key: &str,
    tuple: (i32, &T),
    overwrite: bool,
) -> anyhow::Result<bool> {
    let t = kvp::TupleKeyValue {
        k: tuple.0,
        v: serialize(tuple.1)?,
    };
    let req = kvp::KeyVecInsertQuery {
        key: key.to_string(),
        value: Some(t),
        overwrite: overwrite,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::KeyVecInsertResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "KeyVecInsert", buf)
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?
            .as_slice(),
    )?;
    Ok(res.success)
}

pub fn keyvec_get<'de, T: Deserialize<'de>>(
    binding_name: &'static str,
    key: &str,
) -> HandlerResult<Vec<(i32, T)>> {
    let req = kvp::KeyVecGetQuery {
        key: key.to_string(),
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::KeyVecGetResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "KeyVecGet", buf)?
            .as_slice(),
    )?;

    let result: Vec<(i32, T)> = res
        .values
        .into_iter()
        .map(|t| (t.k, deserialize(t.v.as_slice()).unwrap()))
        .collect();
    Ok(result)
}

pub fn keyvec_remove_item(
    binding_name: &'static str,
    key: &str,
    value_idx: i32,
) -> HandlerResult<()> {
    let req = kvp::KeyVecRemoveItemQuery {
        key: key.to_string(),
        value_idx: value_idx,
    };
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf).expect("req encoded error");
    let _res = kvp::KeyVecRemoveItemResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "KeyVecRemoveItem", buf)?
            .as_slice(),
    )?;

    Ok(())
}

pub fn keyvec_tail_off(
    binding_name: &'static str,
    key: &str,
    remain: usize,
) -> HandlerResult<usize> {
    let req = kvp::KeyVecTailOffQuery {
        key: key.to_string(),
        remain: remain as u32,
    };
    let mut buf = Vec::with_capacity(req.encoded_len() as usize);
    req.encode(&mut buf).expect("req encoded error");
    let res = kvp::KeyVecTailOffResponse::decode(
        untyped::host(binding_name)
            .call(CAPABILITY, "KeyVecTailOff", buf)?
            .as_slice(),
    )?;
    Ok(res.len as usize)
}
