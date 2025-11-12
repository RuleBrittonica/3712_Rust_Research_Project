# REM-server

This provides a simple JSON-RPC server for handling REM commands from an
external source (e.g. the REM-VSCode extension).
The server is under active development and may change in future releases.
The server communicates over stdin/stdout using a custom JSON-RPC protocol. It
expects one JSON-RPC request per line, and responds with one JSON-RPC response
per request.

All REM components needed are installed automatically when installing REM-sever
using cargo. To install REM-server, run the following command:

```bash
cargo install rem-server
```