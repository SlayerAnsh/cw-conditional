use std::{collections::HashMap, str::from_utf8};

use cosmwasm_std::{from_slice, Deps, Env, Int128, WasmQuery};
use cw_json::JSON;
use serde_cw_value::{to_value, Value};
use serde_json_wasm::to_string;

use crate::{
    packages::eval::eval::{evaluate, Tokens},
    state::{CONDITION, VARIABLES},
    types::{
        Condition, ConditionCompare, ConditionCtx, ConditionWing, CurrentQueryCtx, ExternalQuery,
        ExternalQueryMsg, InwardExecuteCtx, Variable,
    },
};

/** Queries */

/** Utilities */

pub fn create_condition_ctx(env: Env, execute_ctx: Option<InwardExecuteCtx>) -> JSON {
    let ctx = ConditionCtx {
        execute_ctx: execute_ctx.clone(),
        query_ctx: CurrentQueryCtx { env: env },
    };
    let mut ctx = JSON::from_any(ctx);

    // Parse the msg binary to json so it can also be used for references
    match execute_ctx {
        Some(exc_ctx) => {
            ctx.update("execute_ctx.msg", to_value(&exc_ctx.msg).unwrap())
                .unwrap();
        }
        None => {}
    };
    ctx
}

pub fn evaluate_default_condition(deps: &Deps, ctx: &JSON) -> bool {
    let condition = CONDITION.load(deps.storage).unwrap();
    evaluate_condition(&deps, &ctx, condition)
}

pub fn evaluate_condition(deps: &Deps, ctx: &JSON, condition: Condition) -> bool {
    let left = match condition.left {
        ConditionWing::Expression(tokens) => evaluate_expressions(deps, ctx, &tokens),
        ConditionWing::Number(v) => Some(Value::String(v.to_string())),
        ConditionWing::String(v) => Some(Value::String(v)),
        ConditionWing::Bool(v) => Some(Value::Bool(v)),
        ConditionWing::Condition(c) => Some(Value::Bool(evaluate_condition(deps, ctx, *c))),
    }
    .unwrap();
    let left = match left.clone() {
        Value::String(v) => v,
        _ => to_string(&left).unwrap(),
    };
    println!("LEFT = {left:?}");
    let right = match condition.right {
        ConditionWing::Expression(tokens) => evaluate_expressions(deps, ctx, &tokens),
        ConditionWing::Number(v) => Some(Value::String(v.to_string())),
        ConditionWing::String(v) => Some(Value::String(v)),
        ConditionWing::Bool(v) => Some(Value::Bool(v)),
        ConditionWing::Condition(c) => Some(Value::Bool(evaluate_condition(deps, ctx, *c))),
    }
    .unwrap();
    let right = match right.clone() {
        Value::String(v) => v,
        _ => to_string(&right).unwrap(),
    };
    println!("RIGHT = {right:?}");

    if left.parse::<Int128>().is_ok() && right.parse::<Int128>().is_ok() {
        let left = left.parse::<Int128>().unwrap();
        let right = right.parse::<Int128>().unwrap();
        match condition.compare {
            ConditionCompare::Eq => left.eq(&right),
            ConditionCompare::Neq => left.ne(&right),
            ConditionCompare::Lt => left.lt(&right),
            ConditionCompare::Lte => left.le(&right),
            ConditionCompare::Gt => left.gt(&right),
            ConditionCompare::Gte => left.ge(&right),
        }
    } else {
        match condition.compare {
            ConditionCompare::Eq => left.eq(&right),
            ConditionCompare::Neq => left.ne(&right),
            // All other conditions are not valid for this type of operator
            _ => Err("Invalid operator for non integer types").unwrap(),
        }
    }
}

fn evaluate_expressions(deps: &Deps, ctx: &JSON, tokens: &Tokens) -> Option<Value> {
    if tokens.len() == 1 {
        return match evaluate_token(deps, ctx, tokens.first().unwrap()) {
            Some(value) => Some(value),
            None => Some(to_value(tokens.first().unwrap()).unwrap()),
        };
    }
    let mut variables = HashMap::<&str, Int128>::new();
    tokens
        .iter()
        .for_each(|token| match evaluate_token(deps, ctx, token) {
            Some(value) => {
                variables.insert(&token, to_string(&value).unwrap().parse().unwrap());
            }
            None => {}
        });
    Some(to_value(evaluate(tokens.clone(), &variables).unwrap().to_string()).unwrap())
}

pub fn evaluate_token(deps: &Deps, ctx: &JSON, token: &String) -> Option<Value> {
    if let Some(variable) = VARIABLES.may_load(deps.storage, &token).unwrap() {
        match evaluate_variable(deps, ctx, &variable) {
            Some(value) => Some(value),
            None => None,
        }
    } else {
        None
    }
}

pub fn evaluate_variable(deps: &Deps, ctx: &JSON, variable: &Variable) -> Option<Value> {
    match variable {
        Variable::Raw(raw) => Some(to_value(raw).unwrap()),
        Variable::Reference(reference) => ctx.get(reference.as_str()).and_then(|v| Some(v.clone())),
        Variable::Query(query) => {
            match evaluate_query(deps, ctx, query) {
                Some(value) => {
                    match &query.result {
                        Some(key) => {
                            // Key is provided, we need to fetch the key from the json or any value returned
                            let json = JSON::from(value);
                            json.get(&key).and_then(|v| Some(v.clone()))
                        }
                        None => Some(value),
                    }
                }
                _ => None,
            }
        } // _ => None,
    }
}

fn evaluate_query(deps: &Deps, ctx: &JSON, query: &ExternalQuery) -> Option<Value> {
    match evaluate_variable(deps, ctx, &query.contract).unwrap() {
        Value::String(contract) => {
            let address = deps.api.addr_validate(&contract.as_str()).unwrap();
            match &query.query {
                ExternalQueryMsg::Raw(msg) => {
                    match deps
                        .querier
                        .query_wasm_raw(address.clone(), msg.key.as_bytes())
                    {
                        Ok(data) => match data {
                            Some(data) => {
                                let res = from_utf8(&data).unwrap();
                                let res = from_slice(res.as_bytes()).unwrap();
                                return Some(res);
                            }
                            None => {
                                return None;
                            }
                        },
                        Err(err) => Err(format!(
                            "Smart query failed for contract {address} with error - {err:?}"
                        ))
                        .unwrap(),
                    }
                }
                ExternalQueryMsg::Smart(msg) => {
                    let query_msg = WasmQuery::Smart {
                        contract_addr: address.to_string(),
                        msg: msg.msg.clone(),
                    }
                    .into();
                    match deps.querier.query(&query_msg) {
                        Ok(data) => Some(data),
                        Err(err) => Err(format!(
                            "Smart query failed for contract {address} and msg {msg:?} with error - {err:?}"
                        ))
                        .unwrap(),
                    }
                }
            }
        }
        _ => Err("Invalid contract for query").unwrap(),
    }
}
