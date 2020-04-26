# mini-kv

A demo key-value database.

## Design (Not Implemented yet)

### Server

The server stores the data in a plain `HashMap<Key, Value>`.

Also writes into a log file for each operation.

### Client

The client visit the server via grpc.
