use crate::wascc_actor as actor;
use actor::prelude::*;
use codec::messaging;
use codec::messaging::BrokerMessage;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use tea_codec;

// pub type ActionCallback = FnMut(&BrokerMessage) -> HandlerResult<()>;
lazy_static! {
    pub static ref MAP_HANDLER: Mutex<
        HashMap<
            String,
            Box<dyn FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static>,
        >,
    > = Mutex::new(HashMap::new());
}

pub fn get_uuid() -> String {
    let extras = extras::default();
    extras.get_guid().unwrap()
}

pub fn result_handler(msg: &BrokerMessage, uuid: &str) -> HandlerResult<()> {
    //info!("receive nats message: {:?}", msg);
    let callback = {
        match MAP_HANDLER.lock() {
            Ok(mut hash_map) => hash_map.remove(uuid),
            Err(e) => {
                error!("Result handler lock failed, details: {:?}", e);
                None
            }
        }
    };
    match callback {
        Some(mut callback) => callback(&msg),
        None => {
            error!("Cannot find callback function from hashmap. Cannot callbck");
            Ok(())
        }
    }
}

pub fn call<F>(subject: &str, reply_to: &str, param: Vec<u8>, callback: F) -> HandlerResult<()>
where
    F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
{
    let uuid = get_uuid();
    //info!("uuid -> {}", &uuid);

    let reply = format!("{}.{}", reply_to, uuid);
    // TODO error here when tpm & layer1 both call, how to fix?
    MAP_HANDLER.lock().unwrap().insert(uuid, Box::new(callback));

    if let Err(e) = untyped::default().call(
        "wascc:messaging",
        messaging::OP_PUBLISH_MESSAGE,
        serialize(BrokerMessage {
            subject: subject.to_string(),
            reply_to: reply,
            body: param,
        })?,
    ) {
        error!("actor calls nats provider publish error {}", e);
    }

    Ok(())
}

pub fn call_intercom<F>(subject: &str, reply_to: &str, param: Vec<u8>, callback: F) -> HandlerResult<()>
    where
        F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
{
    let uuid = get_uuid();

    let reply = format!("{}.{}", reply_to, uuid);
    MAP_HANDLER.lock().unwrap().insert(uuid, Box::new(callback));

    if let Err(e) = untyped::default().call(
        "tea:intercom",
        messaging::OP_PUBLISH_MESSAGE,
        serialize(BrokerMessage {
            subject: subject.to_string(),
            reply_to: reply,
            body: param,
        })?,
    ) {
        error!("actor calls intercom provider publish error {}", e);
    }

    Ok(())
}

pub fn delay_call<F>(
    subject: &str,
    param: Vec<u8>,
    delay_seconds: u64,
    callback: F,
) -> HandlerResult<()>
where
    F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
{
    let uuid = get_uuid();

    let subject = format!("{}.{}", subject, uuid);
    // TODO error here when tpm & layer1 both call, how to fix?
    MAP_HANDLER.lock().unwrap().insert(uuid, Box::new(callback));

    if let Err(e) = untyped::default().call(
        "wascc:messaging",
        tea_codec::OP_DELAY_PUBLISH,
        serialize(tea_codec::DelayMessage {
            delay_seconds,
            subject: subject.to_string(),
            reply_to: "".to_string(),
            body: param,
        })?,
    ) {
        error!("actor ra calls nats provider publish error {}", e);
    }
    Ok(())
}
