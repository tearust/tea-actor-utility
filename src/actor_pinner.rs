use crate::action;
use prost::Message;
use wascc_actor::HandlerResult;

pub fn is_node_ready<F>(reply_to: &str, mut ready_callback: F) -> anyhow::Result<()>
where
    F: FnMut(bool) -> HandlerResult<()> + Sync + Send + 'static,
{
    action::call(
        "actor.pinner.intercom.is_node_ready",
        reply_to,
        Vec::new(),
        move |msg| {
            let ready: bool = tea_codec::deserialize(msg.body.as_slice())?;
            ready_callback(ready)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_key1<F>(deployment_id: &str, reply_to: &str, mut callback: F) -> anyhow::Result<()>
where
    F: FnMut(Option<Vec<u8>>) -> HandlerResult<()> + Sync + Send + 'static,
{
    action::call(
        &format!("actor.pinner.intercom.get_key1.{}", deployment_id),
        reply_to,
        Vec::new(),
        move |msg| {
            let res: Option<Vec<u8>> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_description_cid<F>(
    deployment_id: &str,
    reply_to: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>) -> HandlerResult<()> + Sync + Send + 'static,
{
    action::call(
        &format!(
            "actor.pinner.intercom.get_description_cid.{}",
            deployment_id
        ),
        reply_to,
        Vec::new(),
        move |msg| {
            let res: Option<String> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_code_or_data_cid<F>(
    deployment_id: &str,
    reply_to: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>) -> HandlerResult<()> + Sync + Send + 'static,
{
    action::call(
        &format!(
            "actor.pinner.intercom.get_code_or_data_cid.{}",
            deployment_id
        ),
        reply_to,
        Vec::new(),
        move |msg| {
            let res: Option<String> = tea_codec::deserialize(msg.body.as_slice())?;
            callback(res)
        },
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn get_deployment_info<F>(
    deployment_id: &str,
    reply_to: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(Option<String>, Option<String>, Option<Vec<u8>>) -> HandlerResult<()>
        + Sync
        + Send
        + 'static,
{
    action::call(
        &format!(
            "actor.pinner.intercom.get_deployment_info.{}",
            deployment_id
        ),
        reply_to,
        Vec::new(),
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
