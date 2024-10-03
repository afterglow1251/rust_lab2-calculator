pub const MODE_COMMON: &str = "--mode common";
pub const MODE_POLISH: &str = "--mode polish";
pub const EXIT_COMMAND: &str = "q";

#[derive(Debug)]
pub enum Mode {
    Common,
    Polish,
}