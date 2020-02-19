use regex::Regex;
use std::io;
use std::io::Read;
use std::string::String;

enum ReaderArg {
    Raw(String),
    FilePath(String),
    None,
}

struct StdinReader {
    buffer: Vec<char>,
}

struct TestReader {
    buffer: Vec<char>,
}

impl StdinReader {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl TestReader {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

trait Reader {
    fn read(&mut self, arg: ReaderArg) -> io::Result<()>;
    fn get_buffer(&self) -> Vec<char>;
}

impl Reader for StdinReader {
    fn read(&mut self, arg: ReaderArg) -> io::Result<()> {
        let mut temp_buf: String = String::new();
        std::io::stdin().read_line(&mut temp_buf)?;
        self.buffer = temp_buf
            .chars()
            .filter(|c| ![&'\n', &'\r'].contains(&c))
            .collect::<Vec<char>>();

        Ok(())
    }

    fn get_buffer(&self) -> Vec<char> {
        self.buffer.clone()
    }
}

impl Reader for TestReader {
    fn read(&mut self, arg: ReaderArg) -> io::Result<()> {
        match arg {
            ReaderArg::Raw(s) => {
                self.buffer = s
                    .chars()
                    .filter(|c| ![&'\n', &'\r'].contains(&c))
                    .collect::<Vec<char>>();
            }
            _ => panic!("Invalid argument!"),
        }

        Ok(())
    }

    fn get_buffer(&self) -> Vec<char> {
        self.buffer.clone()
    }
}

struct StdoutWriter {}
struct TestWriter {
    output: String,
}

impl TestWriter {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

trait Writer {
    fn write(&mut self, output: String) {}
    fn writeln(&mut self, output: String) {}
}

impl Writer for StdoutWriter {
    fn write(&mut self, output: String) {
        print!("{}", output);
    }

    fn writeln(&mut self, output: String) {
        println!("{}", output);
    }
}

impl Writer for TestWriter {
    fn write(&mut self, output: String) {
        self.output = format!("{}{}", self.output, output);
    }

    fn writeln(&mut self, output: String) {
        self.output = format!("{}\n{}", self.output, output);
    }
}

#[derive(Debug)]
struct Cradle<'a, R: Reader, W: Writer> {
    lookahead: Option<char>,
    source: Vec<char>,
    counter: usize,
    reader: R,
    writer: Box<&'a mut W>,
}

impl<'a, R: Reader, W: Writer> Cradle<'a, R, W> {
    fn new(reader: R, writer: &'a mut W) -> Self {
        Self {
            lookahead: None,
            source: Vec::new(),
            counter: 0,
            reader: reader,
            writer: Box::new(writer),
        }
    }

    fn init(&mut self) {
        self.read();
        self.lookahead = Some(self.source[self.counter]);
    }

    fn read(&mut self) -> io::Result<()> {
        self.source = self.reader.get_buffer();
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

    fn error(&mut self, message: String) {
        self.writer.write(format!("\nError: {}", message));
    }

    fn abort(&mut self, message: String) {
        self.error(message);
        panic!("^^^^^^^");
    }

    fn expected(&mut self, expected: String) {
        self.abort(format!("{} expected", expected));
    }

    fn emit(&mut self, s: String) {
        self.writer.write(format!("{}", s));
    }

    fn emit_line(&mut self, s: String) {
        self.writer.writeln(format!("{}", s));
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
    let mut reader = StdinReader::new();
    reader.read(ReaderArg::None);
    let mut writer = StdoutWriter {};
    let mut cradle = Cradle::new(reader, &mut writer);
    cradle.init();
    cradle.expression();
    Ok(())
}

#[cfg(test)]
mod CradleTests {
    use super::*;

    #[test]
    fn given_single_term_expression_output_move_instruction() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1")));
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(String::from("\nMOVE #1,D0\nMOVE D0,D1"), writer.output);
    }

    #[test]
    fn given_add_operation_output_add_instructions() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1+2")));
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,D1\nMOVE #2,D0\nADD D1,D0"),
            writer.output
        );
    }

    #[test]
    fn given_subtract_operation_output_subtract_instructions() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1-2")));
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,D1\nMOVE #2,D0\nSUB D1,D0\nNEG D1"),
            writer.output
        );
    }

    #[test]
    fn given_multiple_operators_output_correct_assembly() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1-2+3-4+7")));
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,D1\nMOVE #2,D0\nSUB D1,D0\nNEG D1\nMOVE #3,D0\nADD D1,D0\nMOVE #4,D0\nSUB D1,D0\nNEG D1\nMOVE #7,D0\nADD D1,D0"),
            writer.output
        );
    }
}
