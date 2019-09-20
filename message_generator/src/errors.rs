use std::{error, fmt};

#[derive(Debug)]
pub enum ParserError {
    TypeInvalid,
    CharSizeInvalid,
    TypeNotFound,
    SizeNotFound,
    BoundsInvalid,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParserError::TypeInvalid => write!(f, "ParserError: type invalid!"),
            ParserError::CharSizeInvalid => write!(f, "ParserError: chars size invalid!"),
            ParserError::TypeNotFound => write!(f, "ParserError: type not found!"),
            ParserError::SizeNotFound => write!(f, "ParserError: size not found!"),
            ParserError::BoundsInvalid => write!(f, "ParserError: bounds invalid!"),
        }
    }
}

// impl fmt::Debug for ParserError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         std::fmt::Display::fmt(self, f)
//     }
// }

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
