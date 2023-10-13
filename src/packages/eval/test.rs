#[cfg(test)]
mod tests {
    use cosmwasm_std::Int128;

    use crate::packages::eval::eval::{evaluate, Tokens};
    use std::collections::HashMap;

    fn tokenize(expr: &str) -> Tokens {
        expr.split_whitespace().map(|t| t.to_string()).collect()
    }

    #[test]
    fn test_evaluate() {
        let mut variables = HashMap::new();
        variables.insert("a", Int128::from(3));
        variables.insert("b", Int128::from(4));

        // Test basic arithmetic expressions
        assert_eq!(evaluate(tokenize("10 + 2 * 6"), &variables), Ok(22.into()));
        assert_eq!(
            evaluate(tokenize("100 * 2 + 12"), &variables),
            Ok(212.into())
        );
        assert_eq!(
            evaluate(tokenize("100 * ( 2 + 12 )"), &variables),
            Ok(1400.into())
        );

        // // Test expressions with negative numbers
        assert_eq!(evaluate(tokenize("-5 + 3"), &variables), Ok((-2).into()));
        assert_eq!(evaluate(tokenize("-5 - 5"), &variables), Ok((-10).into()));
        assert_eq!(evaluate(tokenize("10 - -2"), &variables), Ok(12.into()));

        // // Test expressions with division by zero
        // assert_eq!(
        //     evaluate(tokenize("5 / 0"), &variables),
        //     Err("Division by zero")
        // );

        // Test expressions with invalid operators
        assert_eq!(
            evaluate(tokenize("10 & 2"), &variables),
            Err("Invalid operator")
        );

        // Test expressions with mismatched parentheses
        assert_eq!(
            evaluate(tokenize("( 5 + 2"), &variables),
            Err("Invalid expression")
        );
        // assert_eq!(evaluate(tokenize("5 + 2 )"), &variables), Err("Invalid expression"));
        assert_eq!(
            evaluate(tokenize("( ( 5 + 2 ) * 3"), &variables),
            Err("Invalid expression")
        );

        // Test variables
        assert_eq!(
            evaluate(tokenize("( ( a + b ) * 7 )"), &variables),
            Ok(49.into())
        );
    }
}
