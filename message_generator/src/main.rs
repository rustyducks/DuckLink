use std::fs::File;
use std::io::prelude::*;
use std::fs;
use termion::color;

mod c_generator;
mod cpp_generator;
mod errors;
mod generator;
mod message;
mod parser;
mod python_generator;

use c_generator::CGenerator;
use cpp_generator::CPPGenerator;
use generator::Generator;
use python_generator::PythonGenerator;

fn main() -> Result<(), Vec<String>> {
    //let lang = "python";
    //let lang = "C";
    let lang = "CPP";
    let filename = "messages.toml";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let messages = parser::parse_toml(&contents)?;

    let files = match lang {
        "python" => PythonGenerator::generate_code(messages),
        "C" => CGenerator::generate_code(messages),
        "CPP" => CPPGenerator::generate_code(messages),
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
        let path = format!("../lib/{}/messages/{}", lang, f);
        let mut file = File::create(path).map_err(|e| {
            println!("{}", e);
            vec!["Fail to create file!".to_string()]
            })?;
        file.write_all(&txt.into_bytes()).map_err(|_e| vec!["Fail write file!".to_string()])?;
    }

    



    Ok(())
}




// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }
