use regex::Regex;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Cradle {
    lookahead: Option<char>,
    source: Vec<char>,
    counter: usize,
}

impl Cradle {
    fn new() -> Self {
        Self {
            lookahead: None,
            source: Vec::new(),
            counter: 0,
        }
    }

    fn init(&mut self) {
        self.read();
        self.lookahead = Some(self.source[self.counter]);
    }

    fn read(&mut self) -> io::Result<()> {
        let mut temp_buf: String = String::new();
        std::io::stdin().read_line(&mut temp_buf)?;
        self.source = temp_buf
            .chars()
            .filter(|c| c != &'\n')
            .collect::<Vec<char>>();
        Ok(())
    }

    fn get_char(&mut self) -> io::Result<()> {
        if self.source.len() < 1 {
            self.read().unwrap();
        }
        if self.counter >= self.source.len() - 1 {
            self.lookahead = None;
            return Ok(());
        }
        self.counter += 1;
        self.lookahead = Some(self.source[self.counter]);
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
        if self.lookahead.unwrap() == *x {
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
        if !self.is_alpha(&self.lookahead.unwrap()) {
            self.expected(String::from("Name"));
        }

        let name = self.lookahead.unwrap();
        self.get_char().unwrap();
        name
    }

    fn get_num(&mut self) -> char {
        if !self.is_digit(&self.lookahead.unwrap()) {
            self.expected(String::from("Integer"));
        }

        let num = self.lookahead;
        self.get_char().unwrap();
        num.unwrap()
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
        self.emit_line(String::from("SUB D1,D0"));
        self.emit_line(String::from("NEG D1"));
    }

    fn expression(&mut self) {
        while self.lookahead != None {
            match self.lookahead {
                Some(c) => match c {
                    '+' => self.add(),
                    '-' => self.subtract(),
                    _ => {
                        self.term();
                        self.emit_line(String::from("MOVE D0,D1"));
                        self.expression();
                    }
                },
                None => (),
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut cradle = Cradle::new();
    cradle.init();
    cradle.expression();
    Ok(())
}
