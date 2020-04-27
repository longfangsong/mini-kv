# mini-kv

A demo key-value database.

## Design (Current, may change later)

### Server

The server stores the data in a plain `HashMap<Key, Value>`.

Also writes into a log file for each write operation.

### Client

The client visit the server via grpc.

The client in this repo is a redis-cli-like tool to visit the data on the server.

Use the grpc interface in programs.
