use crate::vm;
use std::io::Read;

#[derive(Debug)]
pub enum BrainFuckError {
    IOError(std::io::Error),
    NoMatchingEndToLoop(usize, usize),
    NoMatchingStartToLoop(usize, usize),
}

pub fn execute<R: Read>(code: &[u8], input: &mut R) -> Result<Vec<u8>, BrainFuckError> {
    let mut line = 1;
    let mut column = 1;

    let mut index = 0;
    let mut output = Vec::new();

    let memory = vm::Memory::new();
    let mut pointer = vm::Pointer::new(&memory);

    let mut scopes = Vec::new();

    while index < code.len() {
        match code[index] {
            b'>' => pointer.right(),
            b'<' => pointer.left(),
            b'+' => pointer.inc(),
            b'-' => pointer.dec(),
            b'.' => output.push(pointer.get()),
            b',' => {
                let mut byte = [0];
                input.read_exact(&mut byte)?;
                pointer.set(byte[0]);
            }
            b'[' => {
                if pointer.get() == 0 {
                    // Find matching
                    let mut count = 0;
                    let start_line = line;
                    let start_column = column;
                    index += 1;
                    loop {
                        if index == code.len() {
                            return Err(BrainFuckError::NoMatchingEndToLoop(
                                start_line,
                                start_column,
                            ));
                        }

                        if code[index] == b']' {
                            if count == 0 {
                                break;
                            } else {
                                count -= 1;
                            }
                        } else if code[index] == b'[' {
                            count += 1;
                        } else if code[index] == b'\n' {
                            line += 1;
                            column = 0;
                        }

                        column += 1;
                        index += 1;
                    }
                } else {
                    scopes.push((index, line, column));
                }
            }
            b']' => {
                if pointer.get() == 0 {
                    scopes.pop();
                } else {
                    match scopes.get(scopes.len() - 1) {
                        Some((start, start_line, start_column)) => {
                            index = *start;
                            line = *start_line;
                            column = *start_column;
                        }
                        None => return Err(BrainFuckError::NoMatchingStartToLoop(line, column)),
                    }
                }
            }
            b'\n' => {
                line += 1;
                column = 0;
            }
            _ => {}
        }

        column += 1;
        index += 1;
    }

    Ok(output)
}

impl From<std::io::Error> for BrainFuckError {
    fn from(error: std::io::Error) -> Self {
        BrainFuckError::IOError(error)
    }
}

impl std::error::Error for BrainFuckError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BrainFuckError::IOError(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for BrainFuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainFuckError::IOError(error) => write!(f, "I/O Error - {}", error),
            BrainFuckError::NoMatchingEndToLoop(line, column) => {
                write!(f, "No matching end to loop for {}:{}", line, column)
            }
            BrainFuckError::NoMatchingStartToLoop(line, column) => {
                write!(f, "No matching start to loop for {}:{}", line, column)
            }
        }
    }
}

#[test]
fn simple() {
    let code = b"++>++[-<+>]<.";

    let output = execute(code, &mut std::io::stdin()).unwrap();

    assert_eq!(&output, &[4]);
}

#[test]
fn hello_world_long() {
    let code = include_bytes!("./hello_world.bf");

    let output = execute(code, &mut std::io::stdin()).unwrap();

    assert_eq!(&output, b"Hello World!\n");
}

#[test]
fn hello_world_compact() {
    let code = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let output = execute(code, &mut std::io::stdin()).unwrap();

    assert_eq!(&output, b"Hello World!\n");
}

#[test]
fn simple_interpreter_test() {
    let code = b">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.";

    let output = execute(code, &mut std::io::stdin()).unwrap();

    assert_eq!(&output, b"Hello World!\n");
}
