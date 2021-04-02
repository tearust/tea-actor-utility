use byteorder::{ByteOrder, LittleEndian};
use wascc_actor::prelude::*;

pub fn get_outbound_sequence() -> anyhow::Result<u32> {
    match untyped::default().call(vmh_codec::VMH_CAPABILITY_ID, "OutboundSequence", vec![]) {
        Ok(res) => {
            let val = LittleEndian::read_u32(res.as_slice());
            Ok(val)
        }
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    }
}
