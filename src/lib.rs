//! Tea Project WaSCC Actor Utility Supporting Binary and Sorted Vector Key-Value Pair and Macros
//!  to Build Nats Message Actor Handler
//!
//!WaSCC Actors are supposed to be stateless. Global variable and any kind of storage across handler
//! calls are not recommended (although technically doable). Host provided key-value pair is one of
//! the handy storage shared across handler functions. WaSCC provided Redis provider and a sample
//! key-value pair provider.
//!
//! There are a few reasons I cannot use them:
//!
//! - Redis is over kill.
//! - Existing key-value pair provider use `String`, I prefer to use `Vec<u8>`
//! - I need a `Sorted Vec` value type besides the 4 existing types.
//! - Writing the code direct call from actor is kind of cumbersome. Need an additional actor utility
//!  layer in between.
//!
//! There is one more important reason for this later: To use macro to make Nats message actor handler
//! code clean. See README.md for code sample.
//!
//!The actor handler code will look like this
//! ```
//! actor_handlers!{
//!   codec::messaging::OP_DELIVER_MESSAGE => handle_nats_message,
//!   codec::core::OP_HEALTH_REQUEST => health
//! }
//! actor_messaging_handlers!{
//!   ("http", _, "bootstrap", "sync_from_other_actor",..) => sync_from_other_actor,
//!   ("http", _, "api", "request_active_nodes",..) => request_active_nodes,
//!   //code snipet...
//! }
//! ```
//! Please note the "`_`" act as the "`*`" in the Nats subscription wildcard as well as the "`..`" act
//! as "`>`" in Nats subject. Think about writing code to handle those `*` and `>` wildcard without the macro.
//! So I made this library to scrach on my own itch. It can probably help you as well.
//!
//! # About the Tea Project
//!
//! Tea Project (Trusted Execution & Attestation) is a Wasm runtime build on top of RoT(Root of Trust)
//! from both trusted hardware environment and blockchain technologies. Developer, Host and Consumer
//! do not have to trust any others to not only protecting privacy but also preventing cyber attacks.
//! The execution environment under remoted attestation can be verified by blockchain consensys.
//! Crypto economy is used as motivation that hosts are willing run trusted computing nodes. This
//! platform can be used by CDN providers, IPFS Nodes or existing cloud providers to enhance existing
//! infrastructure to be more secure and trustless.
//!
//! Introduction [blog post](https://medium.com/@pushbar/0-of-n-cover-letter-of-the-trusted-webassembly-runtime-on-ipfs-12a4fd8c4338)
//!
//! Project [repo](http://github.com/tearust). More and more repo will be exposed soon.
//!
//! Yet to come //! project site [( not completed yet) http://www.t-rust.com/](http://www.t-rust.com/)
//!
//! Contact: kevin.zhang.canada_at_gmail_dot_com.
//!
//! We are just started, all kinds of help are welcome!
//!

pub extern crate wascc_actor;
// use prost::Message;

//use std::env;
mod env_proto {
    include!(concat!(env!("OUT_DIR"), "/env.rs"));
}
pub mod kvp_proto {
    include!(concat!(env!("OUT_DIR"), "/kvp.rs"));
}
pub mod tpm_provider_proto {
    include!(concat!(env!("OUT_DIR"), "/tpm.rs"));
}
pub mod actor_ra_proto {
    include!(concat!(env!("OUT_DIR"), "/actor_ra.rs"));
}
pub mod p2p_proto {
    include!(concat!(env!("OUT_DIR"), "/p2p.rs"));
}
mod crypto_proto {
    include!(concat!(env!("OUT_DIR"), "/crypto.rs"));
}
pub mod actor_pinner_proto {
    include!(concat!(env!("OUT_DIR"), "/actor_pinner.rs"));
}

pub mod action;
pub mod actor_crypto;
pub mod actor_env;
pub mod actor_ipfs;
pub mod actor_kvp;
pub mod actor_nats;
pub mod actor_pinner;
pub mod actor_util;
pub mod async_collector;
pub mod common;
pub mod ipfs_p2p;
pub mod layer1;
pub mod vmh;

#[macro_use]
extern crate log;
extern crate lazy_static;

pub fn encode_protobuf<T>(protobuf_type: T) -> anyhow::Result<Vec<u8>>
where
    T: prost::Message,
{
    let mut buf: Vec<u8> = Vec::with_capacity(protobuf_type.encoded_len());
    protobuf_type.encode(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {}
