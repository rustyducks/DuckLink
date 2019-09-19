use toml::value::Value;
use crate::errors::ParserError;

#[derive(Debug)]
pub struct MsgSpec {
    pub name: String,
    pub fields: Vec<(String, Type)>
}

impl MsgSpec {
    pub fn new(name: String) -> MsgSpec {
        MsgSpec {
            name,
            fields : Vec::new()
        }
    }
}

//static DEFAULT_CHARS_SIZE: usize = 10;

#[derive(Debug)]
pub enum Type {
    I8, I16, I32,
    U8, U16, U32,
    F32,
    CHARS(usize)
}

impl Type {

    const DEFAULT_CHARS_SIZE: usize = 10;

    fn from_string(s: &str) -> Result<Type, ParserError> {
        match s.as_ref() {
            "i8" => Ok(Type::I8),
            "i16" => Ok(Type::I16),
            "i32" => Ok(Type::I32),
            "u8" => Ok(Type::U8),
            "u16" => Ok(Type::U16),
            "u32" => Ok(Type::U32),
            "f32" => Ok(Type::F32),
            "chars" => Ok(Type::CHARS(Type::DEFAULT_CHARS_SIZE)),
            _ => {
                println!("Type {} invalid.", s);
                Err(ParserError::TypeInvalid)
            }
        }
    }

    pub fn from_toml(raw: &Value) -> Result<Type, ParserError> {
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
}
