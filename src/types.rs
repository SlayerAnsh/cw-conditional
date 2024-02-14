use crate::packages::eval::eval::Tokens;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, Env, Int128};

#[cw_serde]
pub struct InwardExecuteCtx {
    pub env: Env,
    pub msg: Binary,
    pub funds: Vec<Coin>,
    pub sender: Addr,
    pub original_sender: Addr,
}

#[cw_serde]
pub struct CurrentQueryCtx {
    pub env: Env,
}
#[cw_serde]
pub struct ConditionCtx {
    pub execute_ctx: Option<InwardExecuteCtx>,
    pub query_ctx: CurrentQueryCtx,
}

#[cw_serde]
pub struct ExternalQueryRawMsg {
    pub key: String,
}

#[cw_serde]
pub struct ExternalQuerySmartMsg {
    pub msg: Binary,
}

#[cw_serde]
pub enum ExternalQueryMsg {
    Raw(ExternalQueryRawMsg),
    Smart(ExternalQuerySmartMsg),
}

#[cw_serde]
pub struct ExternalQuery {
    pub contract: Variable,
    pub query: ExternalQueryMsg,
    pub result: Option<String>,
}

#[cw_serde]
pub enum Variable {
    Raw(String),
    Reference(String),
    Query(Box<ExternalQuery>),
}

#[cw_serde]
pub enum ConditionWing {
    Number(Int128),
    String(String),
    Expression(Tokens),
    Bool(bool),
    Condition(Box<Condition>),
}

#[cw_serde]
pub enum ConditionCompare {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[cw_serde]
pub struct Condition {
    pub left: ConditionWing,
    pub right: ConditionWing,
    pub compare: ConditionCompare,
}
