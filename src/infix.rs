use crate::utils::math::{push_current_number, precedence, apply_operation};

pub(crate) fn evaluate_infix_expression(expr: &str) -> Result<f64, String> {
    let mut values = vec![];
    let mut ops = vec![];
    let mut current_number = String::new();
    let mut last_char: Option<char> = None; // Store the last character processed

    for ch in expr.chars() {
        if ch.is_digit(10) || ch == '.' {
            current_number.push(ch);
        } else if ch == '(' {
            ops.push(ch);
        } else if ch == ')' {
            if !current_number.is_empty() {
                push_current_number(&mut current_number, &mut values)?; // Push the number
            }
            while let Some(op) = ops.pop() {
                if op == '(' {
                    break; // Found a matching parenthesis
                }
                let right = values.pop().ok_or("Not enough values".to_string())?;
                let left = values.pop().ok_or("Not enough values".to_string())?;
                let result = apply_operation(left, right, op)?;
                values.push(result);
            }
        } else if "+-*/".contains(ch) {
            if ch == '-' && (last_char.is_none() || "+-*/(".contains(last_char.unwrap())) {
                // If '-'' is the first character or after the operator, treat it as a sign
                current_number.push('-'); // Add a sign to the number
            } else {
                if !current_number.is_empty() {
                    push_current_number(&mut current_number, &mut values)?; // Push a number before the operator
                }
                while let Some(&last_op) = ops.last() {
                    if precedence(last_op) >= precedence(ch) {
                        let right = values.pop().ok_or("Not enough values".to_string())?;
                        let left = values.pop().ok_or("Not enough values".to_string())?;
                        let result = apply_operation(left, right, last_op)?;
                        values.push(result);
                        ops.pop();
                    } else {
                        break;
                    }
                }
                ops.push(ch); // Add the operator
            }
        } else if ch.is_whitespace() {
            continue; // Skip spaces
        } else {
            return Err("Unknown symbol".to_string());
        }

        last_char = Some(ch); // Update the last processed character
    }

    if !current_number.is_empty() {
        push_current_number(&mut current_number, &mut values)?; // Push the last number
    }

    while let Some(op) = ops.pop() {
        let right = values.pop().ok_or("Not enough values".to_string())?;
        let left = values.pop().ok_or("Not enough values".to_string())?;
        let result = apply_operation(left, right, op)?;
        values.push(result);
    }

    values.pop().ok_or("Calculation error".to_string())
}
