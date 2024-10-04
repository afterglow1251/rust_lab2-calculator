extern crate meval;

pub(crate) fn evaluate_infix_expression(expr: &str) -> Result<f64, String> {
    meval::eval_str(expr).map_err(|e| format!("Evaluation error: {}", e))
}
