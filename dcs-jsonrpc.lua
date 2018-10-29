--
-- load  JSON
--
local jsonlib = lfs.writedir() .. "Scripts\\GAW\\json.lua"
local json = loadfile(jsonlib)()

--
-- load and start dcs-jsonrpc
--
package.loaded["dcsjsonrpc"] = nil
local jsonrpc = require "dcsjsonrpc"
jsonrpc.start()

--
-- RPC methods
--

function method_health()
    return {
        result = "\"ok\""
    }
end

function method_outText(params)
    env.info("[JSON-RPC] 7")
    -- TODO: return error on missing params
    trigger.action.outText(params.text, params.displayTime, params.clearView)

    return nil
end

--
-- RPC request handler
--
function handleRequest(method, params)
    env.info("[JSON-RPC] receiving method "..method.." with params: "..tostring(params))

    local fnName = "method_"..method
    env.info("[JSON-RPC] 1")
    local fn = _G[fnName]
    env.info("[JSON-RPC] 2")
    if params ~= nil then
        env.info("[JSON-RPC] 3")
        params = json:decode(params)
    end
    env.info("[JSON-RPC] 4")

    if type(fn) == "function" then
        env.info("[JSON-RPC] 5")
        local ok, result = pcall(fn, params)
        if not ok then
            env.info("[JSON-RPC] error executin "..method.." with params: "..tostring(params)..": "..tostring(result))
        end

        env.info("[JSON-RPC] 8")
        return result
    else
        env.info("[JSON-RPC] 6")
        return {
            error = "unsupported method "..method
        }
    end
end

--
-- execute JSON-RPC requests every 0.1 seconds
--
timer.scheduleFunction(function(arg, time)
    while jsonrpc.next(handleRequest) do
        -- TODO: restrict handled requests per tick?
    end

    return timer.getTime() + .1 -- return time of next call
end, nil, timer.getTime() + .1)

--
-- listen to DCS events
--
local eventHandler = {}
function eventHandler:onEvent(event)
    if event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
        jsonrpc.broadcast("player_enter_unit", json:encode({
            initiator = event.initiator:getName(),
        }))
    end
end

world.addEventHandler(eventHandler)