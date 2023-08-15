pub const LEVEL_DEBUG: usize = 0;
pub const LEVEL_INFO: usize = 1;
pub const LEVEL_WARN: usize = 2;
pub const LEVEL_ERROR: usize = 3;
const LEVELS: [&str; 4] = ["\x1b[36mDEBUG\x1b[0m", "\x1b[32mINFO\x1b[0m", "\x1b[33mWARN\x1b[0m", "\x1b[31mERROR\x1b[0m"];

pub fn log(level_index: usize, message: &str) {
    println!("{} {}", format!("\x1b[1m{:>21}\x1b[0m", LEVELS[level_index]), message.to_string());
}
