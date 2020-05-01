# mini-kv

A demo key-value database.

## How to use & How to build

Please view [API doc](./doc/api.md).

## Design (Current, may change later)

### Server

The server stores the data in a plain `HashMap<Key, Value>`.

Plan to change this into lsm-tree.

Also writes into a redo-log file for each write operation.

### Client

The client visit the server via grpc.

The client in this repo is a redis-cli-like tool to visit the data on the server.

Use the grpc interface in programs.

### Design detail

Please view [design doc](./doc/design.md)

### benchmark

put:get:delete:scan = 4:4:1:1

1600079 request in 319 seconds.

QPS: 5015

For compare, normal Redis's QPS is around 45000.
