#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, ensure};
use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg,
    ado_contract::{
        permissioning::{is_context_permissioned},
        ADOContract,
    },
    common::context::ExecuteContext,
    error::ContractError,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};


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
        .add_attribute("owner", info.sender)
        )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    if let ExecuteMsg::AMPReceive(pkt) = msg {
        ADOContract::default().execute_amp_receive(
            ctx,
            pkt,
            handle_execute,
        )
    } else {
        handle_execute(ctx, msg)
    }
}

pub fn handle_execute(
    ctx: ExecuteContext,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
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
        
        _ => ADOContract::default().execute(ctx, msg)
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        
        _ => ADOContract::default().query(deps, env, msg),
    }
}

#[cfg(test)]
mod tests {}
