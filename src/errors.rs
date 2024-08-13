pub fn print_error_token_line(line_number: usize, token: char) {
    eprintln!(
        "[line {}] Error: Unexpected character: {token}",
        line_number
    );
}
