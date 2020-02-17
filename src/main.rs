use std::io;
use std::io::Read;

fn read() -> io::Result<char> {
    let mut temp_buf: String = String::new();
    std::io::stdin().read_line(&mut temp_buf)?;
    let c: char = temp_buf.chars().collect::<Vec<char>>()[0];
    Ok(c)
}

fn get_char<'a>(buffer: &'a mut char) -> io::Result<()> {
    *buffer = read().unwrap();
    Ok(())
}

fn main() -> io::Result<()> {
    let mut lookahead: char = '0';
    get_char(&mut lookahead)?;
    println!("{}", lookahead);
    Ok(())
}
