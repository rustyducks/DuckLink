use std::fs;
use termion::color;

mod c_generator;
mod errors;
mod generator;
mod message;
mod parser;
mod python_generator;

use c_generator::CGenerator;
use generator::Generator;
use python_generator::PythonGenerator;

fn main() -> Result<(), std::io::Error> {
    let lang = "python";
    let filename = "messages.toml";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let messages = parser::parse_toml(&contents)?;

    let files = match lang {
        "python" => PythonGenerator::generate_code(messages),
        "C" => CGenerator::generate_code(messages),
        _ => panic!("{} not supported!", lang),
    };

    for (f, txt) in files {
        println!(
            "{}{}\n----------------------------{}",
            color::Fg(color::Blue),
            f,
            color::Fg(color::Reset)
        );
        println!("{}\n", txt);
    }

    Ok(())
}
