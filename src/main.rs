mod assembly_interpreter;
mod compiler;
mod reader;
mod writer;
use compiler::Compiler;
use reader::{Reader, ReaderArg, StdinReader};
use std::io;
use writer::{StdoutWriter, Writer};

fn main() -> io::Result<()> {
    let mut reader = StdinReader::new();
    reader.read(ReaderArg::None).unwrap();
    let mut writer = StdoutWriter {};
    let mut cradle = Compiler::new(reader, &mut writer);
    cradle.init();
    cradle.program();
    Ok(())
}
