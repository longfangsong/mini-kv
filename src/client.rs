use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::error::Error;
use crate::Result;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Key value store client
pub struct KvsClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(KvsClient {
            reader: BufReader::new(tcp_reader),
            writer: BufWriter::new(tcp_writer),
        })
    }

    /// Get the value of a given key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        bincode::serialize_into(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let resp = bincode::deserialize_from(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(msg) => Err(Error::StringError(msg)),
        }
    }

    /// Set the value of a string key in the server.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        bincode::serialize_into(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let resp = bincode::deserialize_from(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(msg) => Err(Error::StringError(msg)),
        }
    }

    /// Remove a string key in the server.
    pub fn remove(&mut self, key: String) -> Result<()> {
        bincode::serialize_into(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let resp = bincode::deserialize_from(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(msg) => Err(Error::StringError(msg)),
        }
    }
}
