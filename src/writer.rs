pub struct StdoutWriter {}
pub struct TestWriter {
    pub output: String,
}

impl TestWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

pub trait Writer {
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
