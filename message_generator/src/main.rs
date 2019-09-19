
use std::fs;
use toml::Value;
use toml::value::Table;

#[derive(Debug)]
enum Type {
    I8, I16, I32,
    U8, U16, U32,
    F32,
    CHARS,
    ERR
}

impl Type {
    fn python_default(&self) -> &str {
        match *self {
            Type::I8 | Type::I16 | Type::I32 => "0",
            Type::U8 | Type::U16 | Type::U32 => "0",
            Type::F32 => "0.0",
            Type::CHARS => "b''",
            Type::ERR => panic!("Type::ERR should not be matched at this point (removed in parse_message_class).")
        }
    }
}

#[derive(Debug)]
struct DuckSpec {
    name: String,
    fields: Vec<(String, Type)>
}

impl DuckSpec {
    fn new(name: String) -> DuckSpec {
        DuckSpec {
            name,
            fields : Vec::new()
        }
    }
    
    fn to_python(&self) -> String {
        let mut code = format!(
"class {}: DuckMsg()
\tdef __init__(self):
", self.name);
        if self.fields.len() == 0 {
            code.push_str("\t\tpass\n")
        } else {
            self.fields.iter().map(|(field_name, t)| format!("\t\tself.{} = {}\n", field_name, t.python_default())).map(|x| code.push_str(&x)).for_each(drop);
        }
        
        code.push_str("\n");
        code
    }
}

fn main() -> Result<(), std::io::Error> {

    let filename = "../messages.toml";
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    parse_toml(&contents)?;
    
    Ok(())
}

fn parse_toml(contents: &str) -> Result<(), std::io::Error> {
    let value = contents.parse::<Value>()?;
    
    let up_messages = &value["up"];     
    let down_messages = &value["down"];
    
    let mut messages = vec![];
    
    if let Value::Table(t) = up_messages {
        messages = parse_message_class(t)?;
    }
    
    if let Value::Table(t) = down_messages {
        let mut down_messages = parse_message_class(t)?;
        messages.append(&mut down_messages);
    }
    
    
    
    for msg in messages {
        let code = msg.to_python();
        println!("{}", code);
    }
    
    
    
    
    
    
    
    
    
    
    
    
    Ok(())   
}

fn parse_message_class(t: &Table) -> Result<Vec<DuckSpec>, std::io::Error> {

    let mut messages = vec![];
    
    for (msg_name, m) in t {
        if let Value::Table(msg_table) = m {
            let mut msg = DuckSpec::new(msg_name.to_string());
            for (name, typ) in msg_table {   
                if let Value::String(t) = typ {
                    let t = match t.as_ref() {
                        "i8" => Type::I8,
                        "i16" => Type::I16,
                        "i32" => Type::I32,
                        "u8" => Type::U8,
                        "u16" => Type::U16,
                        "u32" => Type::U32,
                        "f32" => Type::F32,
                        "chars" => Type::CHARS,
                        _ => Type::ERR
                    };
                    if let Type::ERR = t {
                        println!("type inconnu !");
                    } else {
                        msg.fields.push((name.to_string(), t));
                    }
                }
            }
            messages.push(msg);
        }       
    }
    
    Ok(messages)
}

