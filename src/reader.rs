use std::fs;
use std::io;

#[derive(Debug)]
pub enum ReaderArg {
    Raw(String),
    FilePath(String),
    None,
}

pub struct StdinReader {
    buffer: Vec<char>,
}

pub struct TestReader {
    buffer: Vec<char>,
}

pub struct FileReader {
    buffer: Vec<char>,
}

impl FileReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl Reader for FileReader {
    fn read(&mut self, arg: ReaderArg) -> io::Result<()> {
        match arg {
            ReaderArg::FilePath(file_path) => {
                self.buffer = fs::read_to_string(file_path).unwrap().chars().collect()
            }
            x => panic!("Expected FilePath argument, found {:?}", x),
        }

        Ok(())
    }

    fn get_buffer(&self) -> Vec<char> {
        self.buffer.clone()
    }
}

impl StdinReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl TestReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

pub trait Reader {
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
