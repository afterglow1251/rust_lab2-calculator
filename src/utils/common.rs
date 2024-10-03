use crate::infix::evaluate_infix_expression;
use crate::polish::evaluate_polish_expression;
use crate::{Mode, MODE_COMMON, MODE_POLISH, EXIT_COMMAND};


pub(crate) fn evaluate_expression(expr: &str, mode: &Mode) -> Result<f64, String> {
    match mode {
        Mode::Common => evaluate_infix_expression(expr),
        Mode::Polish => evaluate_polish_expression(expr),
    }
}

pub(crate) fn mode_to_string(mode: &Option<Mode>) -> &'static str {
    match mode {
        Some(Mode::Common) => "Common",
        Some(Mode::Polish) => "Polish notation",
        _ => "No mode selected",
    }
}

pub(crate) fn check_exit_command(input: &str) -> bool {
    input.eq_ignore_ascii_case(EXIT_COMMAND)
}

pub(crate) fn choose_mode(input: &str) -> Option<Mode> {
    match input.to_lowercase().as_str() {
        MODE_COMMON => Some(Mode::Common),
        MODE_POLISH => Some(Mode::Polish),
        _ => None,
    }
}