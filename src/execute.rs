use andromeda_std::{common::context::ExecuteContext, error::ContractError};
use cosmwasm_std::Response;

use crate::{
    state::{CONDITION, VARIABLES},
    types::{Condition, Variable},
};

pub fn add_condition(ctx: ExecuteContext, condition: Condition) -> Result<Response, ContractError> {
    CONDITION.save(ctx.deps.storage, &condition)?;
    Ok(Response::new()
        .add_attribute("method", "add_condition")
        .add_attribute("condition", format!("{condition:?}")))
}

pub fn add_variable(
    ctx: ExecuteContext,
    variable: &Variable,
    name: &str,
) -> Result<Response, ContractError> {
    VARIABLES.save(ctx.deps.storage, name, variable)?;
    Ok(Response::new()
        .add_attribute("method", "add_variable")
        .add_attribute("name", name)
        .add_attribute("condition", format!("{variable:?}")))
}
