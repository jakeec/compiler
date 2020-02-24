mod assembly_interpreter;
mod compiler;
mod reader;
mod writer;
use compiler::Compiler;
use reader::{FileReader, Reader, ReaderArg, StdinReader};
use std::io;
use writer::{StdoutWriter, Writer};

fn main() -> io::Result<()> {
    let mut reader = FileReader::new();
    reader
        .read(ReaderArg::FilePath("./test_data/input.xx".to_string()))
        .unwrap();
    let mut writer = StdoutWriter {};
    let mut cradle = Compiler::new(reader, &mut writer);
    cradle.init();
    cradle.program();
    Ok(())
}
