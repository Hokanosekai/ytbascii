use termion;

pub fn get_shell_dim() -> (u16, u16) {
    let (cols, rows) = termion::terminal_size().unwrap();
    (cols, rows)
}

