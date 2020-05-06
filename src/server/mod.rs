use std::collections::HashMap;

use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::server::storage::KvStorage;
use crate::Result;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// the storage backend of the server
pub mod storage;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `KvStorage`
pub struct KvServer {
    storage: Box<dyn KvStorage>,
}

impl Default for KvServer {
    /// Creates a default `KvStore`.
    /// By now, the background storage would be a HashMap
    fn default() -> Self {
        Self {
            storage: Box::new(HashMap::new()),
        }
    }
}

impl KvServer {
    /// create a new `KvServer`, it's storage backend is `storage`
    pub fn new<S: KvStorage + 'static>(storage: S) -> Result<Self> {
        Ok(KvServer {
            storage: Box::new(storage),
        })
    }

    /// start listening request from `addr`
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let peer_addr = tcp.peer_addr()?;
        let mut reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);
        while let Ok(req) = bincode::deserialize_from::<_, Request>(&mut reader) {
            debug!("Receive request from {}: {:?}", peer_addr, req);
            match req {
                Request::Get { key } => {
                    let result = match self.storage.get(&key) {
                        Ok(value) => GetResponse::Ok(value),
                        Err(e) => GetResponse::Err(format!("{}", e)),
                    };
                    bincode::serialize_into(&mut writer, &result)?
                }
                Request::Set { key, value } => {
                    let result = match self.storage.insert(key, value) {
                        Ok(_) => SetResponse::Ok(()),
                        Err(e) => SetResponse::Err(format!("{}", e)),
                    };
                    bincode::serialize_into(&mut writer, &result)?
                }
                Request::Remove { key } => {
                    let result = match self.storage.remove(&key) {
                        Ok(_) => RemoveResponse::Ok(()),
                        Err(e) => RemoveResponse::Err(format!("{}", e)),
                    };
                    bincode::serialize_into(&mut writer, &result)?
                }
            }
            writer.flush()?;
        }
        Ok(())
    }
}
