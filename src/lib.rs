use std::{io::{Write, Read}, fmt::Display};

use serde::{Serialize, de::DeserializeOwned};

/// An error which is either an IO error or a postcard Error.
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Postcard(postcard::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<postcard::Error> for Error {
    fn from(value: postcard::Error) -> Self {
        Error::Postcard(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => e.fmt(f),
            Error::Postcard(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(e) => e.source(),
            Error::Postcard(_) => None,
        }
    }
}

pub trait BufReadExt: Read {
    /// Reads a value of type T from the reader.
    fn read_serde<T: DeserializeOwned>(&mut self) -> Result<T, Error> {
        let mut len_bytes = [0; 8];
        self.read_exact(&mut len_bytes)?;
        let len = usize::from_le_bytes(len_bytes);
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        let value = postcard::from_bytes(&bytes)?;
        Ok(value)
    }

    fn read_serde_into_writer<W: Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        let mut len_bytes = [0; 8];
        self.read_exact(&mut len_bytes)?;
        writer.write_all(&len_bytes)?;
        let len = usize::from_le_bytes(len_bytes);
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl<R: Read> BufReadExt for R { }

pub trait BufWriteExt: Write {
    /// Writes a value of type T to the writer.
    fn write_serde<T: Serialize>(&mut self, value: T) -> Result<(), Error> {
        let bytes = postcard::to_allocvec(&value)?;
        let len = bytes.len();
        self.write_all(&len.to_le_bytes())?;
        self.write_all(&bytes)?;
        Ok(())
    }
}

impl<W: Write> BufWriteExt for W { }
