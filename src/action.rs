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
    trace!("action result_handler received message: {:?}", msg);
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
) -> anyhow::Result<()>
where
    F: FnMut(&BrokerMessage) -> HandlerResult<()> + Sync + Send + 'static,
{
    let uuid = get_uuid();
    MAP_HANDLER
        .lock()
        .unwrap()
        .insert(uuid.clone(), Box::new(callback));

    if let Err(e) = request_intercom(send_to_actor, reply_actor, uuid, msg) {
        error!("calls asyc intercom error {}", e);
    }
    Ok(())
}

pub fn post_intercom(actor_name: &str, msg: &BrokerMessage) -> anyhow::Result<Vec<u8>> {
    let subject = format!("post.{}", actor_name);
    match untyped::default().call(
        "tea:intercom",
        tea_codec::OP_INTERCOM_MESSAGE,
        serialize_msg(subject, "".into(), msg)?,
    ) {
        Err(e) => Err(anyhow::anyhow!(
            "actor calls intercom provider publish error {}",
            e
        )),
        Ok(r) => Ok(r),
    }
}

pub fn request_intercom(
    actor_name: &str,
    my_actor_name: &str,
    uuid: String,
    mut msg: BrokerMessage,
) -> anyhow::Result<Vec<u8>> {
    if !msg.reply_to.is_empty() {
        return Err(anyhow::anyhow!("When calling request_intercom, always leave reply_to empty, because it is used for response socket"));
    }
    let subject = format!("request.{}", actor_name);
    msg.reply_to = format!("reply.{}.{}", my_actor_name, uuid);
    match untyped::default().call(
        "tea:intercom",
        tea_codec::OP_INTERCOM_MESSAGE,
        serialize_msg(subject, msg.reply_to.clone(), &msg)?,
    ) {
        Err(e) => Err(anyhow::anyhow!(
            "actor calls intercom provider publish error {}",
            e
        )),
        Ok(r) => Ok(r),
    }
}

//to avoid endless ping-pong. we have to have a reply_intercom to end
//when reply an incoming intercom, you cannot call another intercom, you can only
//call reply_intercom so that it will end because no callback function as input parameter
pub fn reply_intercom(subject: &str, body: Vec<u8>) -> anyhow::Result<()> {
    if let Err(e) = untyped::default().call(
        "tea:intercom",
        tea_codec::OP_INTERCOM_MESSAGE,
        serialize(BrokerMessage {
            subject: subject.to_string(),
            reply_to: "".into(),
            body, // the body content is the msg to be delivered
        })
        .map_err(|e| anyhow::anyhow!("{}", e))?,
    ) {
        error!("actor calls intercom provider publish error {}", e);
    }
    Ok(())
}

fn serialize_msg(
    subject: String,
    reply_to: String,
    msg: &BrokerMessage,
) -> anyhow::Result<Vec<u8>> {
    let body = serialize(msg).map_err(|e| anyhow::anyhow!("{}", e))?;
    if body.len() > 1024 * 128 {
        // because of the limitation in intercom message, we pre-check message length here to avoid
        //  error in receiver side
        return Err(anyhow::anyhow!("serialized broker message over than 128K"));
    }
    Ok(serialize(BrokerMessage {
        subject,
        reply_to,
        body, // the body content is the msg to be delivered
    })
    .map_err(|e| anyhow::anyhow!("{}", e))?)
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
