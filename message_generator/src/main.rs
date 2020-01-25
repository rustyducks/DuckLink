use std::fs;
use std::fs::File;
use std::io::prelude::*;
use termion::color;
extern crate clap;
use clap::{Arg, App};
use rand::Rng;

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

    let matches = App::new("Ducklink message generator")
                          .version("0.1")
                          .author("Fabien B. <fabien.bonneval@gmail.com>")
                          .about("Generate messages according to input toml file")
                          .arg(Arg::with_name("FILE")
                               .help("set input toml message file")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("lang")
                               .short("l")
                               .long("lang")
                               .value_name("LANG")
                               .takes_value(true)
                               .multiple(true)
                               .required(true)
                               .help("Languages to generate messages for. Possible values: C, CPP, Python."))
                          .get_matches();


    let msg_file = matches.value_of("FILE").unwrap();

    let contents = fs::read_to_string(msg_file).expect("Something went wrong reading the file");
    let messages = parser::parse_toml(&contents)?;

    let mut rng = rand::thread_rng();
    let UID: u32 = rng.gen();

    for lang in matches.values_of("lang").unwrap() {
        let files = match lang {
            "Python" => PythonGenerator::generate_messages(&messages, UID),
            "C" => CGenerator::generate_messages(&messages, UID),
            "CPP" => CPPGenerator::generate_messages(&messages, UID),
            _ => panic!("{} not supported!", lang),
        };

        for (f, txt) in files {
            println!(
                "{}{}\n----------------------------{}",
                color::Fg(color::Blue),
                f,
                color::Fg(color::Reset)
            );
            //println!("{}\n", txt);
            let path = format!("../lib/{}/messages/{}", lang, f);
            let mut file = File::create(path).map_err(|e| {
                println!("{}", e);
                vec!["Fail to create file!".to_string()]
            })?;
            file.write_all(&txt.into_bytes())
                .map_err(|_e| vec!["Fail write file!".to_string()])?;
        }

    }

    Ok(())
}
