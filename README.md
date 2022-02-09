# Introduction to Rust Tokio async library
This is a toy server-client Rust application that uses `tokio` runtime.
The purspose of this repo is simple introduction to the Rust
`tokio` library different from the `hello Tokio`, `chat app`, or `echo server`. 

This application consists of two parts: `server` and `client`.

### Server
The server listens to `127.0.0.1:8080` and upon accepting a new connection
creates a new task which generates frames (sequence of bytes) and sends them
to the client. A frame has the following format:
```
<n><byte_1>...<byte_n>
```
In other words, a single frame begins with a byte indicating the number
of bytes following the first one. For example, `2 4 6` or `4 1 4 3 2`.
After sending a single byte the server waits for some time between 1 or 5 seconds.
Bytes sent by the server are logged to the `stdout`.

### Client
The client connects to `127.0.0.1:8080` and upon successfull connection
reads bytes sent the server. If a frame is incomplete (for example `3 1 2`) then
it logs the frame is incomplete and waits for the rest of the frames, otherwise it
parses the frame and logs the bytes to the `stdout`.

### How to run
`cargo run --bin server`

`cargo run --bin client`


