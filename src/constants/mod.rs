pub const MODE_COMMON: &str = "--mode common";
pub const MODE_POLISH: &str = "--mode polish";
pub const EXIT_COMMAND: &str = "q";

pub const MEMORY_FLAG: &str = "-R";

#[derive(Debug)]
pub enum Mode {
    Common,
    Polish,
}