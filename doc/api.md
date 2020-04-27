# API

The server exports a grpc api.

You can view the `minikv.proto` [here](../rpc/minikv.proto) to know the grpc interface.

## Cli

### Server

Start the server is very easy, just run it as a normal rust program:

```shell
# from source code
cargo run --bin server
# or from binary
./server
```

You can pass an argument into the program as the redo-log.

```shell
cargo run --bin server ./minikv.log
```

By default, the server runs on `localhost:5884`. You can config this with environment variable `HOST` and `PORT`.

Currently, if no argument is provided, a redo-log file will be generate at `./minikv.log`.

### Client

The client is also just a plain rust program:

```shell
# from source code
cargo run --bin client
# or from binary
./client
```

You can specify the server address in program arguments.

```shell
cargo run --bin client 127.0.0.1:5885
```

After you run this, you'll see the repl interface.

Use following commands to interact with server:

#### `GET`

##### Params

- The key to get.

##### Return

- The value which the key associated to

##### Error

- If no key found in the store, a "key not found" error will occur.

#### `PUT`

##### Params

- The key to put.
- The value to put.

##### Return

Return nothing.

##### Error

No error except internet connection related errors should occur.

#### `DELETE`

##### Params

- The key to delete.

##### Return

Return nothing

##### Error

- If no key found in the store, a "key not found" error will occur.

#### `SCAN`

##### Params

- The cursor used to scan from.

##### Return

- Next cursor to use, if this value is 0, it means all keys have been scanned.
- A set of keys.

##### Error

No error except internet connection related errors should occur.

