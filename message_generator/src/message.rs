use crate::errors::ParserError;
use toml::value::Value;

#[macro_export]
macro_rules! bounds {
    ($name:ident) => {
        Bounds {
            min: std::$name::MIN as i64,
            max: std::$name::MAX as i64,
        }
    };
    ($name:ident; $T:ty) => {
        Bounds {
            min: std::$name::MIN as $T,
            max: std::$name::MAX as $T,
        }
    };
}

macro_rules! set_min_max {
    ($val:path, $t_table:ident, $b:ident, $t:ident) => {{
        let mut new_min = $b.min;
        if let Some(v) = $t_table.get("min") {
            if let $val(min) = v {
                if min >= &$b.min {
                    new_min = *min;
                } else {
                    return Err(ParserError::BoundsInvalid);
                }
            } else {
                return Err(ParserError::BoundsInvalid);
            }
        }

        let mut new_max = $b.max;
        if let Some(v) = $t_table.get("max") {
            if let $val(max) = v {
                if max <= &$b.max {
                    new_max = *max;
                } else {
                    return Err(ParserError::BoundsInvalid);
                }
            } else {
                return Err(ParserError::BoundsInvalid);
            }
        }

        $b.min = new_min;
        $b.max = new_max;

        if new_min < new_max {
            $b.min = new_min;
            $b.max = new_max;
            Ok(())
        } else {
            Err(ParserError::BoundsInvalid)
        }
    }};
}

#[derive(Debug)]
pub struct MsgSpec {
    pub name: String,
    pub id: usize,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub t: Type,
}

#[derive(Debug)]
pub struct Bounds<T> {
    pub min: T,
    pub max: T,
}

#[derive(Debug)]
pub enum Type {
    I8(Bounds<i64>),
    I16(Bounds<i64>),
    I32(Bounds<i64>),
    U8(Bounds<i64>),
    U16(Bounds<i64>),
    U32(Bounds<i64>),
    F32(Bounds<f64>),
    CHARS(usize),
}

impl MsgSpec {
    /// Return needed buffer size for that message.
    /// It includes 2 start bytes, 1 msg_id byte, 1 msg_len byte, and 2 checksum bytes.
    pub fn get_buffer_size(&self) -> usize {
        self.get_payload_size() + 6
    }

    /// Returns the payload size. This does NOT include msg id, len and payload.
    pub fn get_payload_size(&self) -> usize {
        self.fields.iter().map(|f| f.t.get_size()).sum()
    }

    pub fn uid_msg() -> MsgSpec {
        MsgSpec {
            name: "InterMcuUid".to_string(),
            id: 0,
            fields: vec![Field {
                name: "uid".to_string(),
                t: Type::U32(bounds!(u32)),
            }],
        }
    }
}

impl Type {
    const DEFAULT_CHARS_SIZE: usize = 10;

    pub fn get_size(&self) -> usize {
        match self {
            Type::I8(_b) => 1,
            Type::I16(_b) => 2,
            Type::I32(_b) => 4,
            Type::U8(_b) => 1,
            Type::U16(_b) => 2,
            Type::U32(_b) => 4,
            Type::F32(_b) => 4,
            Type::CHARS(size) => *size,
        }
    }

    fn from_string(s: &str) -> Result<Type, ParserError> {
        match s.as_ref() {
            "i8" => Ok(Type::I8(bounds!(i8))),
            "i16" => Ok(Type::I16(bounds!(i16))),
            "i32" => Ok(Type::I32(bounds!(i32))),
            "u8" => Ok(Type::U8(bounds!(u8))),
            "u16" => Ok(Type::U16(bounds!(u16))),
            "u32" => Ok(Type::U32(bounds!(u32))),
            "f32" => Ok(Type::F32(bounds!(f32; f64))),
            "chars" => Ok(Type::CHARS(Type::DEFAULT_CHARS_SIZE)),
            _ => {
                println!("Type {} invalid.", s);
                Err(ParserError::TypeInvalid)
            }
        }
    }

    pub fn from_toml(raw: &Value) -> Result<Type, ParserError> {
        match raw {
            Value::String(s) => Type::from_string(s.as_ref()),
            Value::Table(t_table) => {
                if let Value::String(s) = t_table.get("type").ok_or(ParserError::TypeNotFound)? {
                    let mut t = Type::from_string(s.as_ref())?;
                    match t {
                        Type::I8(ref mut b)
                        | Type::I16(ref mut b)
                        | Type::I32(ref mut b)
                        | Type::U8(ref mut b)
                        | Type::U16(ref mut b)
                        | Type::U32(ref mut b) => {
                            set_min_max!(Value::Integer, t_table, b, t)?;
                            Ok(t)
                        }
                        Type::F32(ref mut b) => {
                            set_min_max!(Value::Float, t_table, b, t)?;
                            Ok(t)
                        }
                        Type::CHARS(_size) => {
                            if let Value::Integer(size) =
                                t_table.get("size").ok_or(ParserError::SizeNotFound)?
                            {
                                if size > &0 {
                                    Ok(Type::CHARS(*size as usize))
                                } else {
                                    Err(ParserError::CharSizeInvalid)
                                }
                            } else {
                                Err(ParserError::CharSizeInvalid)
                            }
                        }
                    }
                } else {
                    Err(ParserError::TypeInvalid)
                }
            }
            _ => Err(ParserError::TypeInvalid),
        }
    }
}
