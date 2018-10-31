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
    return success("ok")
end

function method_execute(params)
    -- TODO: return error on missing params
    local fn, err = loadstring(params.lua)
    if fn then
        local ok, result = pcall(fn)
        if ok then
            if type(result) == "string" then
                return success(result)
            else
                return success(json:encode(result))
            end
        else
            return error("Error executing Lua code: "..result)
        end
    else
        return error("Error loading Lua code: "..err)
    end
end

--
-- RPC trigger actions
--

function method_outText(params)
    -- TODO: return error on missing params
    trigger.action.outText(params.text, params.displayTime, params.clearView)

    return nil
end

--
-- RPC Group methods
--

function method_getGroups(params)
    -- TODO: return error on missing params
    local names = {}
    for i, group in pairs(coalition.getGroups(params.coalition, params.category)) do
        names[i] = group:getName()
    end
    return success(names)
end

function method_groupExists(params)
    -- TODO: return error on missing params
    local group = Group.getByName(params.name)
    if group == nil then
        return success(false)
    else
        return success(group:isExist())
    end
end

function method_getGroupData(params)
    -- TODO: return error on missing params
    local group = Group.getByName(params.name)
    if group == nil then
        return success(nil)
    end

    local countries = {}
    if group:getCoalition() == coalition.side.RED then
        countries = env.mission.coalition.red.country
    else
        countries = env.mission.coalition.blue.country
    end

    local country = group:getUnit(1):getCountry()
    local id = group:getID()
    for _, countryData in pairs(countries) do
        if type(countryData) == 'table' and type(countryData.plane) == 'table' and type(countryData.plane.group) == 'table' then
            for _, groupData in pairs(countryData.plane.group) do
                if groupData.groupId == id then
                    return success(groupData)
                end
            end
        end
    end

    return success(nil)
end

--
-- Helper
--

function success(result)
    return {
        result = json:encode(result)
    }
end

function error(msg)
    return {
        error = msg
    }
end

--
-- RPC request handler
--
function handleRequest(method, params)
    env.info("[JSON-RPC] receiving method "..method.." with params: "..tostring(params))

    local fnName = "method_"..method
    local fn = _G[fnName]
    if params ~= nil then
        params = json:decode(params)
    end

    if type(fn) == "function" then
        local ok, result = pcall(fn, params)
        if not ok then
            env.info("[JSON-RPC] error executing "..method.." with params: "..tostring(params)..": "..tostring(result))
        end

        return result
    else
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
    if event.id == world.event.S_EVENT_SHOT then
        jsonrpc.broadcast("shot", json:encode({
            time = event.time,
            initiator = event.initiator:getName(),
            weapon = event.weapon:getName(),
        }))

    elseif event.id == world.event.S_EVENT_MISSION_END then
        jsonrpc.broadcast("mission_end", json:encode({
            time = event.time,
        }))
        jsonrpc.stop()

    elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
        jsonrpc.broadcast("player_enter_unit", json:encode({
            time = event.time,
            initiator = event.initiator:getName(),
        }))
    end
end

world.addEventHandler(eventHandler)