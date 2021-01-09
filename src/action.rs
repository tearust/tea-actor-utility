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

pub fn call_async_intercom<F>(
    send_to_actor: &str,
    reply_actor: &str,
    msg: BrokerMessage,
    callback: F,
) -> HandlerResult<()>
where
    F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
{
    let uuid = get_uuid();
    let mut msg = msg;
    // there are three sections in replay: first for reply name, second is reply trunck,
    //  and the last is to locate the callback
    msg.reply_to = format!("{}.{}.{}", reply_actor, &msg.reply_to, &uuid);

    MAP_HANDLER.lock().unwrap().insert(uuid, Box::new(callback));

    if let Err(e) = post_intercom(send_to_actor, &msg) {
        error!("calls asyc intercom error {}", e);
    }
    Ok(())
}

pub fn intercom_reply_to(actor_name: &str, reply_to: &str, body: Vec<u8>) -> HandlerResult<()> {
    post_intercom(
        actor_name,
        &BrokerMessage {
            subject: reply_to.to_string(),
            reply_to: "".to_string(),
            body,
        },
    )?;
    Ok(())
}

pub fn post_intercom(actor_name: &str, msg: &BrokerMessage) -> HandlerResult<Vec<u8>> {
    let subject = format!("post.{}", actor_name);
    match untyped::default().call(
        "tea:intercom",
        "IntercomMessage",
        serialize(BrokerMessage {
            subject,
            reply_to: "".into(),
            body: serialize(msg)?, // the body content is the msg to be delivered
        })?,
    ) {
        Err(e) => Err(format!("actor calls intercom provider publish error {}", e).into()),
        Ok(r) => Ok(r),
    }
}

pub fn request_intercom(
    actor_name: &str,
    my_actor_name: &str,
    mut msg: BrokerMessage,
) -> HandlerResult<Vec<u8>> {
    if !msg.reply_to.is_empty() {
        return Err("When calling request_intercom, always leave reply_to empty, because it is used for response socket".into());
    }
    let subject = format!("request.{}", actor_name);
    let uuid = get_uuid();
    msg.reply_to = format!("reply.{}.{}", my_actor_name, uuid);
    match untyped::default().call(
        "tea:intercom",
        "IntercomMessage",
        serialize(BrokerMessage {
            subject,
            reply_to: msg.reply_to.clone(),
            body: serialize(msg)?, // the body content is the msg to be delivered
        })?,
    ) {
        Err(e) => Err(format!("actor calls intercom provider publish error {}", e).into()),
        Ok(r) => Ok(r),
    }
}

//to avoid endless ping-pong. we have to have a reply_intercom to end
//when reply an incoming intercom, you cannot call another intercom, you can only
//call reply_intercom so that it will end because no callback function as input parameter
pub fn reply_intercom(actor_name: &str, msg: &BrokerMessage) -> HandlerResult<()> {
    let subject = format!("reply.{}", actor_name);
    if let Err(e) = untyped::default().call(
        "tea:intercom",
        "IntercomMessage",
        serialize(BrokerMessage {
            subject,
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
