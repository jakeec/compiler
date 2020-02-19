mod reader;
mod writer;
use reader::{Reader, ReaderArg, StdinReader};
use std::io;
use writer::{StdoutWriter, Writer};

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
        self.read().unwrap();
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
        self.writer
            .writeln(format!("\n\x1b[0;31mError: {}\x1b[0m", message));
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

    fn multiply(&mut self) {
        self.match_char(&'*').unwrap();
        self.factor();
        self.emit_line(String::from("MULS (SP)+,D0"));
    }

    fn divide(&mut self) {
        self.match_char(&'/').unwrap();
        self.factor();
        self.emit_line(String::from("MOVE (SP)+,D1"));
        self.emit_line(String::from("DIVS D1,D0"));
    }

    fn term(&mut self) {
        // let num = self.get_num();
        // self.emit_line(format!("MOVE #{},D0", num));
        self.factor();
        if self.lookahead != None && ['*', '/'].contains(&self.lookahead.unwrap()) {
            self.emit_line(String::from("MOVE D0,-(SP)"));
            match self.lookahead {
                Some(c) => match c {
                    '*' => self.multiply(),
                    '/' => self.divide(),
                    _ => self.expected(String::from("Operator")),
                },
                None => (),
            }
        }
    }

    fn factor(&mut self) {
        if self.lookahead.unwrap() == '(' {
            self.match_char(&'(');
            self.expression();
            self.match_char(&')');
        } else {
            let factor = self.get_num();
            self.emit_line(format!("MOVE #{},D0", factor));
        }
    }

    fn add(&mut self) {
        self.match_char(&'+').unwrap();
        self.term();
        self.emit_line(String::from("ADD (SP)+,D0"));
    }

    fn subtract(&mut self) {
        self.match_char(&'-').unwrap();
        self.term();
        self.emit_line(String::from("SUB (SP)+,D0"));
        self.emit_line(String::from("NEG D0"));
    }

    fn expression(&mut self) {
        self.term();
        while self.lookahead != None && ['+', '-'].contains(&self.lookahead.unwrap()) {
            self.emit_line(String::from("MOVE D0,-(SP)"));
            match self.lookahead {
                Some(c) => match c {
                    '+' => self.add(),
                    '-' => self.subtract(),
                    _ => {
                        self.term();
                    }
                },
                None => (),
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = StdinReader::new();
    reader.read(ReaderArg::None).unwrap();
    let mut writer = StdoutWriter {};
    let mut cradle = Cradle::new(reader, &mut writer);
    cradle.init();
    cradle.expression();
    Ok(())
}

#[cfg(test)]
mod cradle_tests {
    use super::*;
    use reader::TestReader;
    use writer::TestWriter;

    #[test]
    fn given_single_term_expression_output_move_instruction() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(String::from("\nMOVE #1,D0"), writer.output);
    }

    #[test]
    fn given_add_operation_output_add_instructions() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1+2"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,-(SP)\nMOVE #2,D0\nADD (SP)+,D0"),
            writer.output
        );
    }

    #[test]
    fn given_subtract_operation_output_subtract_instructions() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1-2"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,-(SP)\nMOVE #2,D0\nSUB (SP)+,D0\nNEG D0"),
            writer.output
        );
    }

    #[test]
    fn given_multiple_operators_output_correct_assembly() {
        let mut reader = TestReader::new();
        reader
            .read(ReaderArg::Raw(String::from("1-2+3-4+7")))
            .unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,-(SP)\nMOVE #2,D0\nSUB (SP)+,D0\nNEG D0\nMOVE D0,-(SP)\nMOVE #3,D0\nADD (SP)+,D0\nMOVE D0,-(SP)\nMOVE #4,D0\nSUB (SP)+,D0\nNEG D0\nMOVE D0,-(SP)\nMOVE #7,D0\nADD (SP)+,D0"),
            writer.output
        );
    }

    #[test]
    fn given_multiply_output_multiply_assembly() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("2*3"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #2,D0\nMOVE D0,-(SP)\nMOVE #3,D0\nMULS (SP)+,D0"),
            writer.output
        );
    }

    #[test]
    fn given_divide_output_divide_assembly() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("2/3"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #2,D0\nMOVE D0,-(SP)\nMOVE #3,D0\nMOVE (SP)+,D1\nDIVS D1,D0"),
            writer.output
        );
    }

    #[test]
    fn given_parentheses_output_correct_assembly() {
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("(1+2)"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Cradle::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        assert_eq!(
            String::from("\nMOVE #1,D0\nMOVE D0,-(SP)\nMOVE #2,D0\nADD (SP)+,D0"),
            writer.output
        );
    }
}
