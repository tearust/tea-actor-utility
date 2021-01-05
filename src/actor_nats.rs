use codec::messaging;
use codec::messaging::BrokerMessage;
use tea_codec::error::TeaError;
use wascc_actor::prelude::*;

pub fn response_reply_to(reply_to: &str, body: Vec<u8>) -> anyhow::Result<()> {
    response_reply_with_subject(reply_to, "", body)
}

pub fn response_reply_with_subject(
    reply_to: &str,
    subject: &str,
    body: Vec<u8>,
) -> anyhow::Result<()> {
    untyped::default()
        .call(
            "wascc:messaging",
            messaging::OP_PUBLISH_MESSAGE,
            serialize(BrokerMessage {
                subject: String::from(subject),
                reply_to: String::from(reply_to),
                body,
            })
            .map_err(|e| TeaError::CommonError(format!("{}", e)))?,
        )
        .map_err(|e| TeaError::CommonError(format!("{}", e)))?;
    Ok(())
}
