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

    fn get_name(&mut self) -> char {
        if !self.is_alpha(&self.lookahead) {
            self.expected(String::from("Name"));
        }

        let name = self.lookahead;
        self.get_char();
        name
    }

    fn get_num(&mut self) -> char {
        if !self.is_digit(&self.lookahead) {
            self.expected(String::from("Integer"));
        }

        let num = self.lookahead;
        self.get_char();
        num
    }

    fn emit(&self, s: String) {
        print!("{}", s);
    }

    fn emit_line(&mut self, s: String) {
        println!("{}", s);
    }

    fn term(&mut self) {
        let num = self.get_num();
        self.emit_line(format!("MOVE #{},D0", num));
    }

    fn add(&mut self) {
        self.match_char(&'+');
        self.term();
        self.emit_line(String::from("ADD D1,D0"));
    }

    fn subtract(&mut self) {
        self.match_char(&'-');
        self.term();
        self.emit_line(String::from("ADD D1,D0"));
    }

    fn expression(&mut self) {
        self.term();
        self.emit_line(String::from("MOVE D0,D1"));
        match self.lookahead {
            '+' => self.add(),
            '-' => self.subtract(),
            _ => self.expected(String::from("Operator")),
        }
    }
}

fn main() -> io::Result<()> {
    let mut cradle = Cradle::new();
    cradle.get_char()?;
    println!("{:?}", cradle);
    cradle.expression();
    Ok(())
}
