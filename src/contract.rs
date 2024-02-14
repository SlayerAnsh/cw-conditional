use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg,
    ado_contract::{permissioning::is_context_permissioned, ADOContract},
    common::{context::ExecuteContext, encode_binary},
    error::ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ensure, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    execute::{add_condition, add_variable},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{
        create_condition_ctx, evaluate_condition, evaluate_default_condition, evaluate_token,
        evaluate_variable,
    },
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:conditional-ado";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let contract = ADOContract::default();

    let resp = contract.instantiate(
        deps.storage,
        env,
        deps.api,
        info.clone(),
        BaseInstantiateMsg {
            ado_type: "conditional-ado".to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            operators: None,
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    Ok(resp
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    match msg {
        ExecuteMsg::AMPReceive(pkt) => {
            ADOContract::default().execute_amp_receive(ctx, pkt, handle_execute)
        }
        _ => handle_execute(ctx, msg),
    }
}

pub fn handle_execute(ctx: ExecuteContext, msg: ExecuteMsg) -> Result<Response, ContractError> {
    ensure!(
        is_context_permissioned(
            ctx.deps.storage,
            &ctx.info,
            &ctx.env,
            &ctx.amp_ctx,
            msg.as_ref()
        )?,
        ContractError::Unauthorized {}
    );

    match msg {
        ExecuteMsg::AddCondition { condition } => add_condition(ctx, condition),
        ExecuteMsg::AddVariable { variable, name } => add_variable(ctx, &variable, &name),
        _ => ADOContract::default().execute(ctx, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Evaluate { ctx } => {
            let condition_ctx = create_condition_ctx(env, ctx);
            encode_binary(&evaluate_default_condition(&deps, &condition_ctx))
        }
        QueryMsg::EvaluateCondition { condition, ctx } => {
            let condition_ctx = create_condition_ctx(env, ctx);
            encode_binary(&evaluate_condition(&deps, &condition_ctx, condition))
        }
        QueryMsg::EvaluateVariable { name, ctx } => {
            let condition_ctx = create_condition_ctx(env, ctx);
            encode_binary(&evaluate_token(&deps, &condition_ctx, &name))
        }
        QueryMsg::EvaluateCustomVariable { variable, ctx } => {
            let condition_ctx = create_condition_ctx(env, ctx);
            encode_binary(&evaluate_variable(&deps, &condition_ctx, &variable))
        }
        _ => ADOContract::default().query(deps, env, msg),
    }
}

#[cfg(test)]
mod tests {}
