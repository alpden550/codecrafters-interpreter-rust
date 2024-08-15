#[derive(Debug)]
pub struct ErrorT {
    line_number: usize,
    token: Option<char>,
}

impl ErrorT {
    pub fn new(line_number: usize, token: Option<char>) -> Self {
        ErrorT { line_number, token }
    }

    pub fn print_error_line(&self) {
        eprintln!(
            "[line {}] Error: Unexpected character: {}",
            self.line_number,
            self.token.unwrap()
        );
    }

    pub fn print_error_string(&self) {
        eprintln!("[line {}] Error: Unterminated string.", self.line_number);
    }
}
