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

// pub fn call_intercom<F>(destination: &str, msg: BrokerMessage, callback: F) -> HandlerResult<()>
//     where
//         F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
// {
//     let uuid = get_uuid();
//     uuid.as_bytes();

//     let mut msg = msg;
//     msg.reply_to = format!("{}.{}", &msg.reply_to, uuid);
//     MAP_HANDLER.lock().unwrap().insert(uuid, Box::new(callback));

//     if let Err(e) = untyped::default().call(
//         "tea:intercom",
//         "IntercomMessage",
//         serialize(BrokerMessage {
//             subject: destination.to_string(),
//             reply_to: "".into(),
//             body: serialize(msg)?, // the body content is the msg to be delivered
//         })?,
//     ) {
//         error!("actor calls intercom provider publish error {}", e);
//     }

//     Ok(())
// }

pub fn request_intercom(destination: &str, my_actor_name: &str, mut msg: BrokerMessage) -> HandlerResult<Vec<u8>>
{
    if ! msg.reply_to.is_empty() {
        return Err("When calling request_intercom, always leave reply_to empty, because it is used for response socket".into())
    }
    let uuid = get_uuid();
    msg.reply_to = format!("reply.{}.{}", my_actor_name, uuid);
    match untyped::default().call(
        "tea:intercom",
        "IntercomMessage",
        serialize(BrokerMessage {
            subject: destination.to_string(),
            reply_to: msg.reply_to.clone(),
            body: serialize(msg)?, // the body content is the msg to be delivered
        })?,
    ) {
        Err(e) => Err(format!("actor calls intercom provider publish error {}", e).into()),
        Ok(r) => Ok(r)
    }
}

//to avoid endless ping-pong. we have to have a reply_intercom to end
//when reply an incoming intercom, you cannot call another intercom, you can only 
//call reply_intercom so that it will end because no callback function as input parameter
pub fn reply_intercom(destination: &str, msg: &BrokerMessage) -> HandlerResult<()>
{
    if let Err(e) = untyped::default().call(
        "tea:intercom",
        "IntercomMessage",
        serialize(BrokerMessage {
            subject: destination.to_string(),
            reply_to: "".into(),
            body: serialize(msg)?, // the body content is the msg to be delivered
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
