use std::{
    error,
    fmt::{self, Display, Formatter},
    result,
};

#[derive(Debug)]
pub struct Error {
    source: Option<Box<dyn error::Error + Send + Sync>>,
    desc: String,
}

impl Error {
    pub fn new(
        source: impl error::Error + Send + Sync + 'static,
        desc: impl Into<String>,
    ) -> Error {
        Error {
            source: Some(Box::new(source)),
            desc: desc.into(),
        }
    }

    pub fn from_desc(desc: impl Into<String>) -> Error {
        Error {
            source: None,
            desc: desc.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(e) => Some(e.as_ref()),
            None => None,
        }
    }
}

pub type Result = result::Result<Vec<String>, Error>;
