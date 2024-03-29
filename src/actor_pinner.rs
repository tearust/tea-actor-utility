use crate::action;
use crate::action::call_async_intercom;
use prost::Message;
use wascc_actor::prelude::codec::messaging::BrokerMessage;

const PINNER_ACTOR_NAME: &'static str = "pinner";

pub fn is_node_ready<F>(reply_actor: &str, mut ready_callback: F) -> anyhow::Result<()>
where
    F: FnMut(bool) -> anyhow::Result<()> + Sync + Send + 'static,
{
    call_async_intercom(
        PINNER_ACTOR_NAME,
        reply_actor,
        BrokerMessage {
            subject: "actor.pinner.intercom.is_node_ready".into(),
            reply_to: "".into(),
            body: Vec::new(),
        },
        move |msg| {
            debug!("is_node_ready get callback messaeg: {:?}", msg);
            let ready: bool = tea_codec::deserialize(msg.body.as_slice())?;
            ready_callback(ready)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_key1<F>(reply_actor: &str, deployment_id: &str, mut callback: F) -> anyhow::Result<()>
where
    F: FnMut(Option<Vec<u8>>) -> anyhow::Result<()> + Sync + Send + 'static,
{
    action::call_async_intercom(
        PINNER_ACTOR_NAME,
        reply_actor,
        BrokerMessage {
            subject: format!("actor.pinner.intercom.get_key1.{}", deployment_id),
            reply_to: "".into(),
            body: Vec::new(),
        },
        move |msg| {
            let res: Option<Vec<u8>> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_description_cid<F>(
    reply_actor: &str,
    deployment_id: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>) -> anyhow::Result<()> + Sync + Send + 'static,
{
    action::call_async_intercom(
        PINNER_ACTOR_NAME,
        reply_actor,
        BrokerMessage {
            subject: format!(
                "actor.pinner.intercom.get_description_cid.{}",
                deployment_id
            ),
            reply_to: "".into(),
            body: Vec::new(),
        },
        move |msg| {
            let res: Option<String> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_code_or_data_cid<F>(
    reply_actor: &str,
    deployment_id: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>) -> anyhow::Result<()> + Sync + Send + 'static,
{
    action::call_async_intercom(
        PINNER_ACTOR_NAME,
        reply_actor,
        BrokerMessage {
            subject: format!(
                "actor.pinner.intercom.get_code_or_data_cid.{}",
                deployment_id
            ),
            reply_to: "".into(),
            body: Vec::new(),
        },
        move |msg| {
            let res: Option<String> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_deployment_info<F>(
    reply_actor: &str,
    deployment_id: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>, Option<String>, Option<Vec<u8>>) -> anyhow::Result<()>
        + Sync
        + Send
        + 'static,
{
    action::call_async_intercom(
        PINNER_ACTOR_NAME,
        reply_actor,
        BrokerMessage {
            subject: format!(
                "actor.pinner.intercom.get_deployment_info.{}",
                deployment_id
            ),
            reply_to: "".into(),
            body: Vec::new(),
        },
        move |msg| {
            let res =
                crate::actor_pinner_proto::GetDeploymentInfoResponse::decode(msg.body.as_slice())?;
            callback(
                res.code_cid.map(|v| v.value),
                res.description_cid.map(|v| v.value),
                res.key1.map(|v| v.value),
            )
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}
