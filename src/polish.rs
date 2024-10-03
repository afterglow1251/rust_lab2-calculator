use crate::utils::math::{apply_operation};

pub(crate) fn evaluate_polish_expression(expr: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut stack = vec![];

    for token in tokens.iter().rev() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else if "+-*/".contains(*token) {
            let left = stack.pop().ok_or("Not enough values for the operation".to_string())?;
            let right = stack.pop().ok_or("Not enough values for the operation".to_string())?;
            let result = apply_operation(left, right, token.chars().next().unwrap())?;
            stack.push(result);
        } else {
            return Err("Incorrect symbol in Polish notation".to_string());
        }
    }

    stack.pop().ok_or("Calculation error".to_string())
}