use std::collections::{HashMap, VecDeque};

use cosmwasm_std::Int128;

pub type Tokens = Vec<String>;

fn precedence(op: &str) -> Int128 {
    match op {
        "+" | "-" => 1.into(),
        "*" | "/" => 2.into(),
        _ => 0.into(),
    }
}

fn apply_op(a: Int128, b: Int128, op: &str) -> Result<Int128, &'static str> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        _ => Err("Invalid operator"), // Handle invalid operator
    }
}

pub fn evaluate(
    mut tokens: Tokens,
    variables: &HashMap<&str, Int128>,
) -> Result<Int128, &'static str> {
    let mut values = VecDeque::new();
    let mut ops = VecDeque::new();

    println!("TOKENS::{tokens:?}");

    while let Some(token) = tokens.first().cloned() {
        tokens = tokens[1..].to_vec();
        println!("OPS::{ops:?}");
        println!("VALUE::{values:?}");
        if let Ok(constant) = token.parse::<Int128>() {
            values.push_back(constant);
        } else if let Some(&value) = variables.get(&token.as_str()) {
            values.push_back(value);
        } else if token == "(" {
            ops.push_back(token);
        } else if token == ")" {
            while let Some(top) = ops.pop_back() {
                if top == "(" {
                    break;
                }
                let val2 = values.pop_back().ok_or("Invalid expression")?;
                let val1 = values.pop_back().ok_or("Invalid expression")?;
                values.push_back(apply_op(val1, val2, &top)?);
            }
        } else {
            while let Some(top) = ops.back().clone() {
                if top == "(" || precedence(&top) < precedence(&token) {
                    break;
                }
                if let Some(op) = ops.pop_back() {
                    let val2 = values.pop_back().ok_or("Invalid expression")?;
                    let val1 = values.pop_back().ok_or("Invalid expression")?;
                    values.push_back(apply_op(val1, val2, &op)?);
                }
            }
            ops.push_back(token);
        }
    }

    while let Some(op) = ops.pop_back() {
        let val2 = values.pop_back().ok_or("Invalid expression")?;
        let val1 = values.pop_back().ok_or("Invalid expression")?;
        values.push_back(apply_op(val1, val2, &op)?);
    }

    values.pop_back().ok_or("Invalid expression")
}
