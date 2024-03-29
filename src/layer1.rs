use crate::action;
use base64;
use prost::Message;
use vmh_codec::message::structs_proto::ra;

pub fn lookup_node_profile<F>(
    ephemeral_id: &[u8],
    reply_to: &str,
    callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(&ra::NodeProfile) -> anyhow::Result<()> + Sync + Send + 'static,
{
    lookup_node_profile_operation(
        ephemeral_id,
        "layer1.async.reply.lookup_node_profile",
        reply_to,
        callback,
    )
}

pub fn lookup_node_profile_by_tea_id<F>(
    tea_id: &[u8],
    reply_to: &str,
    callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(&ra::NodeProfile) -> anyhow::Result<()> + Sync + Send + 'static,
{
    lookup_node_profile_operation(
        tea_id,
        "layer1.async.reply.node_profile_by_tea_id",
        reply_to,
        callback,
    )
}

fn lookup_node_profile_operation<F>(
    param_bytes: &[u8],
    subject: &str,
    reply_to: &str,
    mut callback: F,
) -> anyhow::Result<()>
where
    F: FnMut(&ra::NodeProfile) -> anyhow::Result<()> + Sync + Send + 'static,
{
    action::call(
        subject,
        reply_to,
        base64::encode(&param_bytes).as_bytes().to_vec(),
        move |msg| {
            // info!("looup_node_profile returns msg_as string:{}", &String::from_utf8(msg.body.clone())?);
            let buf = base64::decode(&String::from_utf8(msg.body.clone())?)?;
            let profile = ra::NodeProfile::decode(buf.as_slice())?;

            // info!("looup_node_profile returns profile:{:?}", &profile);
            callback(&profile)
        },
    )
}
