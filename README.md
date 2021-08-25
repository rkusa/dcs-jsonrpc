Unmaintained as superseded by https://github.com/DCS-gRPC/rust-server


---

# DCS JSON-RPC

A JSON-RPC server that runs inside the DCS World mission environment and exposes mission scripting through an [JSON-API 2.0](https://www.jsonrpc.org/specification) over non-blocking TCP.

[Documentation](./crates/jsonrpc/README.md)

---

**Goals:**
- [JSON-API 2.0](https://www.jsonrpc.org/specification) server to enable non-language specific DCS mission scripting
- Non-blocking TCP server (Lua sockets are blocking, thus implement a custom lua module that is non-blocking)

**Upcoming Improvements:**
- Resilient TCP connection (e.g. no automatic reconnects, yet)
- Add more RPC methods

Contributions are welcome, especially adding more RPC methods (they are added in [dcs-jsonrpc.lua](./mod/Scripts/dcs-jsonrpc/dcs-jsonrpc.lua)).

## Crates

- [**client**](./crates/client) - this is a Rust client that wrappes the JSON-RPC calls into a easy to use API
- [**example**](./crates/example) - this is a simple example of how to use the Rust-based client
- [**common**](./crates/common) - this crate includes some structs that are shared between the different sub-projects
- [**jsonrpc**](./crates/jsonrpc) - this is a Lua module that runs the JSON-RPC server inside the DCS World mission environment
- [**repl**](./crates/repl) - this is a simple REPL that can be used to execute Lua in a running DCS Word mission (meant for debugging)

## Installation

1. Edit `DCS World\Scripts\MissionScripting.lua` and uncomment line 18 to 20; the bottom of the file should look like:

    ```lua
    do
        sanitizeModule('os')
        sanitizeModule('io')
        --sanitizeModule('lfs')
        --require = nil
        loadlib = nil
    end
    ```

2. Copy the contents of [`./mod`](`./mod`) to your `Saved Games\DCS.openbeta` folder (there should be `Saved Games\DCS.openbeta\Scripts\dcs-jsonrpc\dcs-jsonrpc.lua` and `Saved Games\DCS.openbeta\Scripts\dcs-jsonrpc\json.lua`) now.

2. Build module with Rust (stable) by running: `cargo build --release`

3. Copy `target/release/dcsjsonrpc.dll` to `Saved Games\DCS.openbeta\Scripts\dcs-jsonrpc\`

3. Create a DCS mission, add a trigger type `Mission Start` with the Action `Do Script` and insert the following script:

    ```lua
    package.cpath = package.cpath..lfs.writedir()..[[Scripts\dcs-jsonrpc\?.dll;]]
    dofile(lfs.writedir()..[[Scripts\dcs-jsonrpc\dcs-jsonrpc.lua]])
    ```

4. That's it

To validate that it is working either connect to the socket as shown in the [Example](#example) below, or by running `cargo run -p example`.

## Development Setup

During development it is recommended to link `dcs-jsonrpc.lua` from the repository into your `Saved Games\DCS.openbeta\Scripts\dcs-jsonrpc\` folder:

```
cd '.\Saved Games\DCS.openbeta\Scripts\dcs-jsonrpc\'
New-Item -ItemType SymbolicLink -Name dcs-jsonrpc.lua -Value M:/Development/dcs-jsonrpc/mod/Scripts/dcs-jsonrpc/dcs-jsonrpc.lua
```

(don't forget to adjust `M:/Development/dcs-jsonrpc` to the path where you have checked out this repository)

## Example

When the mission is started, there should now be a JSON-RPC 2.0 TCP server be running at `127.0.0.1:7777`. It can be simply tested using `netcat` or `telnet`, eg:

```json
nc 127.0.0.1 7777
>> {"jsonrpc":"2.0","method":"health","id":1}
<< {"jsonrpc":"2.0","result":"ok","id":1}
>> {"jsonrpc":"2.0","method":"outText","params":{"text":"Works!","displayTime":5,"clearView":false}}
```

You can also subscribe to events:

```json
nc 127.0.0.1 7777
>> {"jsonrpc":"2.0","method":"subscribe","params":{"name":"player_enter_unit"},"id":2}
<< {"jsonrpc":"2.0","result":"ok","id":2}
<< {"jsonrpc":"2.0","method":"player_enter_unit","params":{"initiator":"Pilot #001"}}
<< {"jsonrpc":"2.0","method":"player_enter_unit","params":{"initiator":"Pilot #002"}}
```

