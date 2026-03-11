use std::io::{stdout, IsTerminal};

#[allow(dead_code)]
pub fn is_piped() -> bool {
    !stdout().is_terminal()
}
