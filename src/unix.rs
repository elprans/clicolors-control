use std::io::{self, IsTerminal};

#[inline(always)]
pub fn is_a_terminal() -> bool {
    io::stdin().is_terminal()
}

#[inline(always)]
pub fn is_a_color_terminal() -> bool {
    is_a_terminal()
}
