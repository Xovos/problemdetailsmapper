use std::{error::{self, Error}, fmt};

#[derive(Debug)]
pub struct MapperError {
    pub message: String,
    pub inner: Option<Box<dyn Error>>
}

impl MapperError {
    pub fn new(value: impl Into<String>, inner: Option<Box<dyn Error>>) -> Self {
        Self { 
            message: value.into(),
            inner
        }
    }
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = &self.message;
        write!(f, "{msg}")
    }
}

impl error::Error for MapperError {}