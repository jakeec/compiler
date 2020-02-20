#[derive(Debug)]
pub struct AssemblyInterpreter {
    d0: Option<isize>,
    d1: Option<isize>,
    stack: Vec<isize>,
    output: Option<isize>,
}

pub enum AssemblyInterpreterError {
    Unexpected,
}

impl AssemblyInterpreter {
    fn new() -> Self {
        Self {
            d0: None,
            d1: None,
            stack: Vec::new(),
            output: None,
        }
    }

    fn eval(&mut self, input: String) -> Result<(), AssemblyInterpreterError> {
        let input = input.chars().collect::<Vec<char>>();
        let input: String = input[1..input.len()].iter().collect();
        let instructions = input
            .split("\n")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        println!("{:?}", instructions);

        for instruction in instructions {
            self.process_instruction(instruction);
        }

        Ok(())
    }

    fn process_instruction(&mut self, instruction: String) -> Result<(), AssemblyInterpreterError> {
        println!("{:?}", self);
        let mut opcode = String::new();
        let mut instruction: Vec<char> = instruction.chars().collect();
        let mut index = 0;
        for i in 0..instruction.len() {
            if instruction[i] == ' ' {
                index = i + 1;
                break;
            }

            opcode.push(instruction[i]);
        }

        let mut rands = &instruction[index..instruction.len()];

        let MOVE: String = String::from("MOVE");
        let ADD: String = String::from("ADD");

        match &opcode[..] {
            "MOVE" => self.move_op(rands),
            "ADD" => self.add_op(rands),
            _ => panic!("Not implemented!"),
        }

        Ok(())
    }

    fn add_op(&mut self, rands: &[char]) {
        use std::convert::TryInto;

        let mut temp: isize = 0;
        let mut i = 0;
        let mut arg_pos = 0;
        for _ in 0..rands.len() {
            if i >= rands.len() {
                break;
            }
            match rands[i] {
                '(' => match &rands[i + 1..i + 5] {
                    &['S', 'P', ')', '+'] => {
                        temp = self.stack.pop().unwrap();
                        i += 5;
                    }
                    x => panic!("Unexpected stack operation: {:?}", x),
                },
                '#' => {
                    temp = (rands[i + 1].to_digit(10).unwrap()).try_into().unwrap();
                    i += 2;
                }
                ',' => {
                    arg_pos += 1;
                    i += 1;
                }
                '-' => match &rands[i + 1..i + 5] {
                    &['(', 'S', 'P', ')'] => {
                        self.stack.push(temp);
                        i += 5;
                    }
                    x => panic!("Unexpected stack operation: {:?}", x),
                },
                'D' => match arg_pos {
                    0 => {
                        match rands[i + 1] {
                            '0' => temp = self.d0.unwrap(),
                            '1' => temp = self.d1.unwrap(),
                            n => panic!("Unknown register D{}", n),
                        }
                        i += 2;
                    }
                    1 => {
                        match rands[i + 1] {
                            '0' => self.d0 = Some(temp + self.d0.unwrap()),
                            '1' => self.d1 = Some(temp + self.d0.unwrap()),
                            n => panic!("Unknown register D{}", n),
                        }
                        i += 2;
                    }
                    x => panic!("Invalid argument position: {}", x),
                },
                x => panic!("Not implemented! {}", x),
            }
        }
    }

    fn move_op(&mut self, rands: &[char]) {
        use std::convert::TryInto;

        let mut temp: isize = 0;
        let mut i = 0;
        let mut arg_pos = 0;
        for _ in 0..rands.len() {
            if i >= rands.len() {
                break;
            }
            match rands[i] {
                '#' => {
                    temp = (rands[i + 1].to_digit(10).unwrap()).try_into().unwrap();
                    i += 2;
                }
                ',' => {
                    arg_pos += 1;
                    i += 1;
                }
                '-' => match &rands[i + 1..i + 5] {
                    &['(', 'S', 'P', ')'] => {
                        self.stack.push(temp);
                        i += 5;
                    }
                    x => panic!("Unexpected stack operation: {:?}", x),
                },
                'D' => match arg_pos {
                    0 => {
                        match rands[i + 1] {
                            '0' => temp = self.d0.unwrap(),
                            '1' => temp = self.d1.unwrap(),
                            n => panic!("Unknown register D{}", n),
                        }
                        i += 2;
                    }
                    1 => {
                        match rands[i + 1] {
                            '0' => self.d0 = Some(temp),
                            '1' => self.d1 = Some(temp),
                            n => panic!("Unknown register D{}", n),
                        }
                        i += 2;
                    }
                    x => panic!("Invalid argument position: {}", x),
                },
                x => panic!("Not implemented! {}", x),
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use crate::reader::{Reader, ReaderArg, TestReader};
    use crate::writer::TestWriter;
    use std::fs;

    #[test]
    fn given_input_split_into_instructions() {
        let mut asm_interp = AssemblyInterpreter::new();
        let mut reader = TestReader::new();
        reader.read(ReaderArg::Raw(String::from("1+2"))).unwrap();
        let mut writer = TestWriter::new();
        let mut cradle = Compiler::new(reader, &mut writer);
        cradle.init();

        cradle.expression();

        asm_interp.eval(writer.output);
        println!("{:?}", asm_interp);
        assert_eq!(asm_interp.d0, Some(3));
    }
}
