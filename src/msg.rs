use andromeda_std::{andr_exec, andr_instantiate, andr_query};
use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::types::{Condition, InwardExecuteCtx, Variable};

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    AddVariable { variable: Variable, name: String },
    AddCondition { condition: Condition },
}

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    Evaluate { ctx: Option<InwardExecuteCtx> },
    #[returns(bool)]
    EvaluateCondition {
        condition: Condition,
        ctx: Option<InwardExecuteCtx>,
    },
    #[returns(String)]
    EvaluateVariable {
        name: String,
        ctx: Option<InwardExecuteCtx>,
    },
    #[returns(String)]
    EvaluateCustomVariable {
        variable: Variable,
        ctx: Option<InwardExecuteCtx>,
    },
}
