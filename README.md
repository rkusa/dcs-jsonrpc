# DCS JSON-RPC

A JSON-RPC server that runs inside the DCS World mission environment and exposes mission scripting through an [JSON-API 2.0](https://www.jsonrpc.org/specification) over non-blocking TCP.

**Goals:**
- [JSON-API 2.0](https://www.jsonrpc.org/specification) server to enable non-language specific DCS mission scripting
- Non-blocking TCP server (Lua sockets are blocking, thus implement a custom lua module that is non-blocking)

**Status:**
Experimental

**Upcoming Improvements:**
- Resilient TCP connection (e.g. no automatic reconnects, yet)
- Server lifecycle (e.g. gracefull shutdown when exiting mission)
- Add more RPC methods

Contributions are welcome, especially adding more RPC methods (they are added in [dcs-jsonrpc.lua](./dcs-jsonrpc.lua)).

## Installation

1. Build module with Rust nightly by running: `cargo build`
2. Edit `DCS World\Scripts\MissionScripting.lua` and uncomment line 18 to 20; the bottom of the file should look like:
    
    ```lua
    do
        sanitizeModule('os')
        sanitizeModule('io')
        --sanitizeModule('lfs')
        --require = nil
        --loadlib = nil
    end
    ```

3. Create a DCS mission, create a trigger type `Mission Start` with the Action `Do Script` and the following script:

    ```lua
    package.cpath = [[M:/Development/dcs-jsonrpc/target/debug/?.dll;]]
    dofile("M:/Development/dcs-jsonrpc/dcs-jsonrpc.lua")
    ```
    
    (don't forget to adjust `M:/Development/dcs-jsonrpc` to the path where you have checked out this repository)

4. That's it

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

## Documentation

[Documentation](./docs)
