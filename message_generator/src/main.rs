
use std::fs;
use toml::Value;
use toml::value::Table;
use std::{error, fmt};

static DEFAULT_CHARS_SIZE: usize = 10;

#[derive(Debug)]
enum ParserError {
    TypeInvalid,
    CharSizeInvalid,
    TypeNotFound,
    SizeNotFound
}

#[derive(Debug)]
enum Type {
    I8, I16, I32,
    U8, U16, U32,
    F32,
    CHARS(usize)
}

struct C;
struct Python;
//struct Rust;

trait Generator<Lang> {
    fn init_variable(&self, name: &str) -> String;
}


impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParserError::TypeInvalid => write!(f, "ParserError: type invalid!"),
            ParserError::CharSizeInvalid => write!(f, "ParserError: chars size invalid!"),
            ParserError::TypeNotFound => write!(f, "ParserError: type not found!"),
            ParserError::SizeNotFound => write!(f, "ParserError: size not found!"),
        }        
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl Type {

    fn from_string(s: &str) -> Result<Type, ParserError> {
        match s.as_ref() {
            "i8" => Ok(Type::I8),
            "i16" => Ok(Type::I16),
            "i32" => Ok(Type::I32),
            "u8" => Ok(Type::U8),
            "u16" => Ok(Type::U16),
            "u32" => Ok(Type::U32),
            "f32" => Ok(Type::F32),
            "chars" => Ok(Type::CHARS(DEFAULT_CHARS_SIZE)),
            _ => {
                println!("Type {} invalid.", s);
                Err(ParserError::TypeInvalid)
            }
        }
    }

    fn from_toml(raw: &Value) -> Result<Type, ParserError> {
        match raw {
            Value::String(s)=> {
                Type::from_string(s.as_ref())
            },
            Value::Table(t_table) => {
                if let Value::String(s) = t_table.get("type").ok_or(ParserError::TypeNotFound)? {
                    let t = Type::from_string(s.as_ref())?;
                    match t {
                        Type::CHARS(_size) => {
                            if let Value::Integer(size) = t_table.get("size").ok_or(ParserError::SizeNotFound)? {
                                if size > &0 {
                                    Ok(Type::CHARS(*size as usize))
                                } else {
                                    Err(ParserError::CharSizeInvalid)
                                }
                            } else {
                                Err(ParserError::CharSizeInvalid)
                            }
                            
                        },
                        _ => Ok(t)
                    }
                } else {
                    Err(ParserError::TypeInvalid)
                }
            },
            _ => Err(ParserError::TypeInvalid)
        }
    }


    fn python_init(&self, name: &str) -> String {
        match *self {
            Type::I8 | Type::I16 | Type::I32 => format!("self.{} = 0", name),
            Type::U8 | Type::U16 | Type::U32 => format!("self.{} = 0", name),
            Type::F32 => format!("self.{} = 0.0", name),
            Type::CHARS(_size) => format!("self.{} = b''", name)
        }
    }

    fn c_init(&self, name: &str) -> String {
        match *self {
            Type::I8 => format!("int8_t {} = 0;", name),
            Type::I16 => format!("int16_t {} = 0;", name),
            Type::I32 => format!("int32_t {} = 0;", name),
            Type::U8 => format!("uint8_t {} = 0;", name),
            Type::U16 => format!("uint16_t {} = 0;", name),
            Type::U32 => format!("uint32_t {} = 0;", name),
            Type::F32 => format!("float32_t {} = 0.0;", name),
            Type::CHARS(size) => format!("char {}[{}] = \"\";", name, size)
        }
    }
}


impl Generator<C> for Type {
        fn init_variable(&self, name: &str) -> String {
        match *self {
            Type::I8 => format!("int8_t {} = 0;", name),
            Type::I16 => format!("int16_t {} = 0;", name),
            Type::I32 => format!("int32_t {} = 0;", name),
            Type::U8 => format!("uint8_t {} = 0;", name),
            Type::U16 => format!("uint16_t {} = 0;", name),
            Type::U32 => format!("uint32_t {} = 0;", name),
            Type::F32 => format!("float32_t {} = 0.0;", name),
            Type::CHARS(size) => format!("char {}[{}] = \"\";", name, size)
        }
    }
}

impl Generator<Python> for Type {
        fn init_variable(&self, name: &str) -> String {
        match *self {
            Type::I8 | Type::I16 | Type::I32 => format!("self.{} = 0", name),
            Type::U8 | Type::U16 | Type::U32 => format!("self.{} = 0", name),
            Type::F32 => format!("self.{} = 0.0", name),
            Type::CHARS(_size) => format!("self.{} = b''", name)
        }
    }
}

#[derive(Debug)]
struct MsgSpec {
    name: String,
    fields: Vec<(String, Type)>
}

impl MsgSpec {
    fn new(name: String) -> MsgSpec {
        MsgSpec {
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
            self.fields.iter().map(|(field_name, t)| format!("\t\t{}\n",(t as &Generator<C>).init_variable(field_name.as_ref()))).map(|x| code.push_str(&x)).for_each(drop);
        }
        
        code.push_str("\n");
        code
    }
}

fn main() -> Result<(), std::io::Error> {

    let filename = "messages.toml";
    
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

fn parse_message_class(t: &Table) -> Result<Vec<MsgSpec>, std::io::Error> {

    let mut messages = vec![];
    
    for (msg_name, m) in t {
        if let Value::Table(msg_table) = m {
            let mut msg = MsgSpec::new(msg_name.to_string());
            for (name, typ) in msg_table {
                match Type::from_toml(typ) {
                    Ok(t) => {
                        msg.fields.push((name.to_string(), t));
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            messages.push(msg);
        }       
    }
    
    Ok(messages)
}
