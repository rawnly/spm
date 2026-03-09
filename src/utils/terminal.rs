use std::io::{stdout, IsTerminal};

pub fn is_piped() -> bool {
    !stdout().is_terminal()
}
