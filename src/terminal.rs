pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
    print!("\x1b[{};{}H", 0, 0);
}
