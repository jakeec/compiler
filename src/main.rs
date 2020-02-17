use regex::Regex;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Cradle {
    lookahead: char,
}

impl Cradle {
    fn new() -> Self {
        Self { lookahead: '0' }
    }

    fn read(&self) -> io::Result<char> {
        let mut temp_buf: String = String::new();
        std::io::stdin().read_line(&mut temp_buf)?;
        let c: char = temp_buf.chars().collect::<Vec<char>>()[0];
        Ok(c)
    }

    fn get_char(&mut self) -> io::Result<()> {
        self.lookahead = self.read().unwrap();
        Ok(())
    }

    fn error(&self, message: String) {
        println!("\nError: {}", message);
    }

    fn abort(&self, message: String) {
        self.error(message);
        panic!("^^^^^^^");
    }

    fn expected(&self, expected: String) {
        self.abort(format!("{} expected", expected));
    }

    fn match_char(&mut self, x: &char) -> io::Result<()> {
        if self.lookahead == *x {
            self.get_char()?;
        } else {
            self.expected(format!("\"{}\"", x));
        }

        Ok(())
    }

    fn is_alpha(&self, x: &char) -> bool {
        x.is_alphabetic()
    }

    fn is_digit(&self, x: &char) -> bool {
        x.is_digit(10)
    }

    fn get_name(&self) -> char {
        if !self.is_alpha(&self.lookahead) {
            self.expected(String::from("Name"));
        }

        self.lookahead
    }

    fn get_num(&self) -> char {
        if !self.is_digit(&self.lookahead) {
            self.expected(String::from("Integer"));
        }

        self.lookahead
    }

    fn emit(&self, s: String) {
        print!("{}", s);
    }

    fn emit_line(&self, s: String) {
        println!("{}", s);
    }
}

fn main() -> io::Result<()> {
    let mut cradle = Cradle::new();
    cradle.get_char()?;
    println!("{:?}", cradle);
    Ok(())
}
