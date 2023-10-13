#[cfg(test)]
mod test {
    use crate::{
        query::{create_condition_ctx, evaluate_condition},
        state::VARIABLES,
        types::{Condition, InwardExecuteCtx, Variable},
    };
    use andromeda_std::common::encode_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::Addr;

    #[test]
    fn test_evaluate_condition() {
        let deps = mock_dependencies();
        let env = mock_env();
        let condition_ctx = create_condition_ctx(env, None);
        let condition = Condition {
            left: crate::types::ConditionWing::String("10".to_string()),
            right: crate::types::ConditionWing::Number(10.into()),
            compare: crate::types::ConditionCompare::Eq,
        };
        let res = evaluate_condition(&deps.as_ref(), &condition_ctx, condition);
        assert_eq!(res, true);
    }

    #[test]
    fn test_evaluate_condition_variable() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let ctx = InwardExecuteCtx {
            env: env.clone(),
            msg: encode_binary(&"".to_string()).unwrap(),
            funds: vec![],
            sender: Addr::unchecked("sender"),
            original_sender: Addr::unchecked("original_sender"),
        };
        let condition_ctx = create_condition_ctx(env, Some(ctx));

        let variable = Variable::Reference("execute_ctx.env.block.height".to_string());
        VARIABLES
            .save(deps.as_mut().storage, "variable", &variable)
            .unwrap();

        let condition = Condition {
            // left: crate::types::ConditionWing::Number(10.into()),
            left: crate::types::ConditionWing::Expression(vec![
                "variable".to_string(),
                "+".to_string(),
                "10000000".to_string(),
            ]),
            right: crate::types::ConditionWing::Expression(vec!["variable".to_string()]),
            compare: crate::types::ConditionCompare::Gt,
        };
        let res = evaluate_condition(&deps.as_ref(), &condition_ctx, condition);
        assert_eq!(res, true);
    }

    #[test]
    fn test_evaluate_condition_variable_mismatch_type() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let ctx = InwardExecuteCtx {
            env: env.clone(),
            msg: encode_binary(&"".to_string()).unwrap(),
            funds: vec![],
            sender: Addr::unchecked("sender"),
            original_sender: Addr::unchecked("original_sender"),
        };
        let condition_ctx = create_condition_ctx(env, Some(ctx));

        let variable = Variable::Reference("execute_ctx.sender".to_string());
        VARIABLES
            .save(deps.as_mut().storage, "variable", &variable)
            .unwrap();

        let condition = Condition {
            left: crate::types::ConditionWing::Expression(vec![
                "10".to_string(),
                "+".to_string(),
                "10000000".to_string(),
            ]),
            right: crate::types::ConditionWing::Expression(vec!["variable".to_string()]),
            compare: crate::types::ConditionCompare::Eq,
        };
        let res = evaluate_condition(&deps.as_ref(), &condition_ctx, condition);
        assert_eq!(res, false);
    }
}
