package.cpath = [[../target/debug/lib?.dylib;../target/debug/?.dll;]]

require 'dcsjsonrpc'
local jsonrpc = require 'dcsjsonrpc'
jsonrpc.start()

function handleRequest(method, params, done)
    if method == "health" then
        return {
            result = "\"ok\""
        }
    else
        return {
            error = "unsupported method "..method
        }
    end
end

while true do
    jsonrpc.next(handleRequest)
end
