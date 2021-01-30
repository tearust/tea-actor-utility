// use codec::messaging;
// use codec::messaging::BrokerMessage;
use prost::Message;
use wascc_actor::prelude::*;

use crate::action;
use crate::actor_ra_proto;
use base64;

pub fn lookup_node_profile<F>(ephemeral_id: &[u8], reply_to: &str, callback: F) -> HandlerResult<()>
where
    F: FnMut(&actor_ra_proto::NodeProfile) -> HandlerResult<()> + Sync + Send + 'static,
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
) -> HandlerResult<()>
where
    F: FnMut(&actor_ra_proto::NodeProfile) -> HandlerResult<()> + Sync + Send + 'static,
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
) -> HandlerResult<()>
where
    F: FnMut(&actor_ra_proto::NodeProfile) -> HandlerResult<()> + Sync + Send + 'static,
{
    action::call(
        subject,
        reply_to,
        base64::encode(&param_bytes).as_bytes().to_vec(),
        move |msg| {
            // info!("looup_node_profile returns msg_as string:{}", &String::from_utf8(msg.body.clone())?);
            let buf = base64::decode(&String::from_utf8(msg.body.clone())?)?;
            let profile = actor_ra_proto::NodeProfile::decode(buf.as_slice())?;

            // info!("looup_node_profile returns profile:{:?}", &profile);
            callback(&profile)
        },
    )
}
