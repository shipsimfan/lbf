use std::io::Write;

mod interpreter;
mod vm;

struct Options {
    output_number: bool,
    input_files: Vec<String>,
}

fn print_error<E: std::error::Error>(error: E) -> ! {
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

#[derive(Debug)]
struct ReadFileError(String, std::io::Error);

#[derive(Debug)]
struct UTF8Error(std::str::Utf8Error);

fn main() {
    let mut parser = argparse::ArgumentParser::<Options>::new();
    parser
        .program_name("LBF")
        .version("1.0")
        .description("Lance Brain Fuck Interpreter")
        .help(true)
        .movable_header("OPTIONAL ARGUMENTS")
        .positional_header("REQUIRED ARGUMENTS");

    parser
        .add_argument("-n", |_, options| {
            options.output_number = true;
            Ok(())
        })
        .help("Display the output as numbers instead of characters")
        .count(0)
        .name("-number")
        .required(false);

    parser
        .add_argument("INPUT FILES", |str, options| {
            options
                .input_files
                .extend(str.iter().map(|str| str.to_owned()));
            Ok(())
        })
        .help("Input files to be run")
        .minimum(1)
        .required(true);

    let options = parser
        .parse_args_env(Options::new())
        .map_err(|e| print_error(e))
        .unwrap();

    for file in options.input_files {
        let code = std::fs::read(&file)
            .map_err(|e| print_error(ReadFileError(file, e)))
            .unwrap();

        let output = interpreter::execute(&code, &mut std::io::stdin())
            .map_err(|e| print_error(e))
            .unwrap();

        if options.output_number {
            let mut i = 0;
            for byte in output {
                print!("{} ", byte);

                i += 1;
                if i == 80 {
                    println!();
                }
            }
        } else {
            print!(
                "{}",
                std::str::from_utf8(&output)
                    .map_err(|e| print_error(UTF8Error(e)))
                    .unwrap()
            );
        }

        std::io::stdout().flush().unwrap();
    }
}

impl Options {
    pub fn new() -> Self {
        Options {
            output_number: false,
            input_files: Vec::new(),
        }
    }
}

impl std::error::Error for ReadFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.1)
    }
}

impl std::fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while reading file '{}' - {}", self.0, self.1)
    }
}

impl std::error::Error for UTF8Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Display for UTF8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while converting output to UTF-8 - {}", self.0)
    }
}
