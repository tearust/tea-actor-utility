use prost::Message;
use serde::{Deserialize, Serialize};
use tea_codec::error::TeaError;
use tea_codec::{deserialize, serialize};
use vmh_codec::message::structs_proto::tokenstate::*;
use wascc_actor::prelude::*;
use tea_codec::TOKENSTATE_CAPABILITY_ID;
pub const OP_START_TXN: &str = "StartTxn";
pub const OP_QUERY_STATE_TSID: &str = "QueryStateTsid";
pub const OP_QUERY_TEA_BALANCE: &str = "QueryTeaBalance";
pub const OP_QUERY_TOKEN_BALANCE: &str = "QueryTokenBalance";
// pub const OP_START_TXN: &str = "StartTxn";
pub const OP_COMMIT_TXN: &str = "CommitTxn";
pub const OP_TOPUP: &str = "Topup";
pub const OP_WITHDRAW: &str = "Withdraw";
pub const OP_MOVE: &str = "Move";

const CAPABILITY: &'static str = "tea:statemachine";

pub fn start_txn() -> HandlerResult<Vec<u8>>{
	let res = StateOperateResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_START_TXN, Vec::new())?
			.as_slice(),
	)?;
	Ok(res.ctx)	
}

pub fn topup(req: TopupRequest) -> HandlerResult<Vec<u8>> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	let res = StateOperateResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_TOPUP, buf)?
			.as_slice(),
	)?;
	Ok(res.ctx)
}

pub fn withdraw(req: WithdrawRequest) -> HandlerResult<Vec<u8>> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	let res = StateOperateResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_WITHDRAW, buf)?
			.as_slice(),
	)?;
	Ok(res.ctx)
}
pub fn mov(req: MoveRequest) -> HandlerResult<Vec<u8>> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	let res = StateOperateResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_MOVE, buf)?
			.as_slice(),
	)?;
	Ok(res.ctx)
}
pub fn commit(req: CommitRequest) -> HandlerResult<Vec<u8>> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	info!("line63");
	let res = StateOperateResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_COMMIT_TXN, buf)?
			.as_slice(),
	)?;
	info!("line69");
	Ok(res.ctx)
}
pub fn query_token_balance(req: QueryTokenBalanceRequest) -> HandlerResult<(Vec<u8>, Vec<u8>)> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	let res = QueryTokenBalanceResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_QUERY_TOKEN_BALANCE, buf)?
			.as_slice(),
	)?;
	Ok((res.balance_bytes, res.state_tsid))
}

pub fn query_tea_balance(req: QueryTeaBalanceRequest) -> HandlerResult<(Vec<u8>, Vec<u8>)> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("req encoded error");
	let res = QueryTeaBalanceResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_QUERY_TEA_BALANCE, buf)?
			.as_slice(),
	)?;
	Ok((res.balance_bytes, res.state_tsid))
}
pub fn query_state_tsid() -> HandlerResult<Vec<u8>> {
	let res = QueryStateTsidResponse::decode(
		untyped::default()
			.call(TOKENSTATE_CAPABILITY_ID, OP_QUERY_STATE_TSID, Vec::new())?
			.as_slice(),
	)?;
	Ok(res.state_tsid)
}