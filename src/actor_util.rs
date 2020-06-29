use std::time::SystemTime;
use wascc_actor::prelude::*;
use prost::Message;
use crate::tpm_provider_proto::*;
use std::error::Error;
use hex;

pub fn verify_ed25519_signature(public_key:Vec<u8>, msg:Vec<u8>, signature:Vec<u8>)->HandlerResult<bool>{
  // info!("==============inside util verify sig function \nsignautre hex is {}\npublic_keyhex{}\nmsghex{}",
  //   hex::encode(&signature),
  //   hex::encode(&public_key),
  //   hex::encode(&msg),
  // );
  
  let req = VerifySignatureRequest{
    signature, public_key, msg
  };

  let buf: Vec<u8> = crate::encode_protobuf(req)?;
  let res = untyped::default().call(
    "tea:tpm",
    "VerifySignature",
    buf
  )?;
  let result = VerifySignatureResponse::decode(res.as_slice())?;

  Ok(result.result)
}

pub fn sign_ed25519_signature(public_key:&Vec<u8>, private_key:&Vec<u8>, message:&Vec<u8>)->HandlerResult<Vec<u8>>{
  let req = SignMessageRequest{
    public_key: public_key.clone(),
    private_key: private_key.clone(), 
    message: message.clone(),
  };

  let buf: Vec<u8> = crate::encode_protobuf(req)?;
  let res = untyped::default().call(
    "tea:tpm",
    "SignMessage",
    buf
  )?;
  let result = SignMessageResponse::decode(res.as_slice())?;

  Ok(result.signature)
}