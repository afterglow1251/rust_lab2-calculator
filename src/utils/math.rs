pub(crate) fn push_current_number(current_number: &mut String, values: &mut Vec<f64>) -> Result<(), String> {
    let num: f64 = current_number.parse().map_err(|_| "Invalid number".to_string())?;
    values.push(num);
    current_number.clear();
    Ok(())
}

pub(crate) fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

pub(crate) fn apply_operation(left: f64, right: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(left + right),
        '-' => Ok(left - right),
        '*' => Ok(left * right),
        '/' => {
            if right == 0.0 {
                Err("Error: division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("Unknown operator".to_string()),
    }
}