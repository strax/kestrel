use std::any::TypeId;
use std::backtrace::Backtrace;
use beef::lean::Cow;
use std::error::{Error as StdError};
use std::fmt;

enum Repr {
    Message(Cow<'static, str>),
    Other(Box<dyn StdError + 'static + Send + Sync>)
}

pub struct Error {
    backtrace: Backtrace,
    repr: Repr
}

impl Error {
    pub fn new<T: StdError + 'static + Send + Sync>(inner: T) -> Error {
        Error {
            backtrace: Backtrace::capture(),
            repr: Repr::Other(Box::new(inner))
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Error");
        d.field("backtrace", &self.backtrace);
        match &self.repr {
            Repr::Message(ref message) => {
                d.field("message", message);
            },
            Repr::Other(ref source) => {
                d.field("source", source);
            }
        }
        d.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Message(message) => f.write_str(message),
            Repr::Other(source) => fmt::Display::fmt(source, f)
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.repr {
            Repr::Other(source) => Some(source.as_ref()),
            _ => None
        }
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}