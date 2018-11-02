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
-- build object id to name tables
--
local groupId2Name = {}
local unitId2Name = {}

for _, coalition in pairs(env.mission.coalition) do
    for _, country in pairs(coalition.country) do
        for _, category in pairs(country) do
            if type(category) == 'table' and type(category.group) == 'table' then
                for _, groupData in pairs(category.group) do
                    local name = env.getValueDictByKey(groupData.name)
                    groupId2Name[groupData.groupId] = name
                    
                    if type(groupData.units) == 'table' then
                        for _, unitData in pairs(groupData.units) do
                            local name = env.getValueDictByKey(unitData.name)
                            unitId2Name[unitData.unitId] = name
                        end
                    end
                end
            end
        end
    end
end

function groupByIdentifier(params)
    local name = params.name
    if type(params.name) ~= "string" then
        name = groupId2Name[params.id]
    end

    if type(name) == "string" then
        return Group.getByName(name)
    else
        return nil
    end
end

function unitByIdentifier(params)
    local name = params.name
    if type(params.name) ~= "string" then
        name = unitId2Name[params.id]
    end

    if type(name) == "string" then
        return Unit.getByName(name)
    else
        return nil
    end
end

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

function method_groupName(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(false)
    else
        return success(group:getName())
    end
end

function method_groupExists(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(false)
    else
        return success(group:isExist())
    end
end

function method_groupData(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
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

function method_groupCoalition(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(nil)
    else
        return success(group:getCoalition())
    end
end

function method_groupCountry(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(nil)
    else
        local unit = group:getUnit(1)
        if unit == nil then
            return success(nil)
        else
            return success(group:getUnit(1):getCountry())
        end
    end
end

function method_groupCategory(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(nil)
    else
        return success(group:getCategory())
    end
end

function method_addGroup(params)
    -- TODO: return error on missing params
    coalition.addGroup(params.country, params.category, params.data)
    -- Note: the group does not exist immediately, why we cannot do something like
    -- group:getName() here
    return success("ok")
end

function method_groupActivate(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group ~= nil then
        group:activate()
    end
    return success("ok")
end

--
-- RPC Unit methods
--

function method_unitName(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(false)
    else
        return success(unit:getID())
    end
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
function idAndName(obj)
    local result = {
        id = tonumber(obj:getID()),
    }
    local name = obj:getName()
    if type(name) == "string" then
        result.name = name
    end
    return result
end

function onEvent(event)
    
    -- Occurs whenever any unit in a mission fires a weapon.
    -- But not any machine gun or autocannon based weapon, those are handled by shooting_start
    if event.id == world.event.S_EVENT_SHOT then
        jsonrpc.broadcast("Shot", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            weapon = { id = event.weapon:getID() },
        }))

    -- Occurs whenever an object is hit by a weapon.
    -- Initiator: The unit object the fired the weapon
    -- Weapon: Weapon object that hit the target
    -- Target: The Object that was hit.
    elseif event.id == world.event.S_EVENT_HIT then
        local target = idAndName(event.target)
        target.category = event.target:getCategory()
        jsonrpc.broadcast("Hit", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            weapon = { id = event.weapon:getID() },
            target = target,
        }))

    -- Occurs when an aircraft takes off from an airbase, farp, or ship.
    -- Initiator: The unit that took off
    -- Place: Object from where the AI took-off from. Can be an Airbase Object, FARP, or Ships
    elseif event.id == world.event.S_EVENT_TAKEOFF then
        jsonrpc.broadcast("Takeoff", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            place = idAndName(event.object),
        }))

    -- Occurs when an aircraft lands at an airbase, farp or ship
    -- Initiator: The unit that has landed
    -- Place: Object that the unit landed on. Can be an Airbase Object, FARP, or Ships
    elseif event.id == world.event.S_EVENT_LAND then
        jsonrpc.broadcast("Land", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            place = idAndName(event.object),
        }))

    -- Occurs when any aircraft crashes into the ground and is completely destroyed.
    -- Initiator: The unit that has crashed
    elseif event.id == world.event.S_EVENT_CRASH then
        jsonrpc.broadcast("Crash", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when a pilot ejects from an aircraft
    -- Initiator: The unit that has ejected
    elseif event.id == world.event.S_EVENT_EJECTION then
        jsonrpc.broadcast("Ejection", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when an aircraft connects with a tanker and begins taking on fuel.
    -- Initiator: The unit that is receiving fuel.
    elseif event.id == world.event.S_EVENT_REFUELING then
        jsonrpc.broadcast("Refueling", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when an object is completely destroyed.
    -- Initiator: The unit that is was destroyed.
    elseif event.id == world.event.S_EVENT_DEAD then
        jsonrpc.broadcast("Dead", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when the pilot of an aircraft is killed.
    -- Can occur either if the player is alive and crashes or if a weapon kills the pilot without completely destroying the plane.
    -- Initiator: The unit that the pilot has died in.
    elseif event.id == world.event.S_EVENT_PILOT_DEAD then
        jsonrpc.broadcast("PilotDead", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when a ground unit captures either an airbase or a farp.
    -- Initiator : The unit that captured the base
    -- Place: The airbase that was captured, can be a FARP or Airbase. When calling place:getCoalition() the faction will already be the new owning faction.
    elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
        jsonrpc.broadcast("BaseCapture", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            place = idAndName(event.place),
        }))

    -- Occurs when a mission starts
    elseif event.id == world.event.S_EVENT_MISSION_START then
        jsonrpc.broadcast("MissionStart", json:encode({
            time = event.time,
        }))

    -- Occurs when a mission ends.
    elseif event.id == world.event.S_EVENT_MISSION_END then
        jsonrpc.broadcast("MissionEnd", json:encode({
            time = event.time,
        }))
        jsonrpc.stop()

    elseif event.id == world.event.S_EVENT_TOOK_CONTROL then
        jsonrpc.broadcast("TookControl", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when an aircraft is finished taking fuel.
    -- Initiator: The unit that was receiving fuel.
    elseif event.id == world.event.S_EVENT_REFUELING_STOP then
        jsonrpc.broadcast("RefuelingStop", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any object is spawned into the mission.
    -- Initiator: The unit that was spawned
    elseif event.id == world.event.S_EVENT_BIRTH then
        jsonrpc.broadcast("Birth", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any system fails on a human controlled aircraft.
    -- Initiator: The unit that had the failure
    elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
        jsonrpc.broadcast("SystemFailure", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))
    
    -- Occurs when any aircraft starts its engines.
    -- Initiator: The unit that is starting its engines.
    elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
        jsonrpc.broadcast("EngineStartup", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any aircraft shuts down its engines.
    -- Initiator: The unit that is stopping its engines
    elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
        jsonrpc.broadcast("EngineShutdown", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any player assumes direct control of a unit.
    -- Initiator: The unit that is being taken control of.
    elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
        jsonrpc.broadcast("PlayerEnterUnit", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any player relieves control of a unit to the AI.
    -- Initiator: The unit that the player left.
    elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
        jsonrpc.broadcast("PlayerLeaveUnit", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_PLAYER_COMMENT then
        jsonrpc.broadcast("PlayerComment", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    -- Occurs when any unit begins firing a weapon that has a high rate of fire. Most common with aircraft cannons (GAU-8), autocannons, and machine guns.
    -- Initiator: The unit that is doing the shooting
    -- Target: The unit that is being targeted.
    elseif event.id == world.event.S_EVENT_SHOOTING_START then
        jsonrpc.broadcast("ShootingStart", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
            target = idAndName(event.target),
        }))

    -- Occurs when any unit stops firing its weapon. Event will always correspond with a shooting start event.
    -- Initiator: The unit that was doing the shooing.
    elseif event.id == world.event.S_EVENT_SHOOTING_END then
        jsonrpc.broadcast("ShootingEnd", json:encode({
            time = event.time,
            initiator = idAndName(event.initiator),
        }))

    end

    -- unimplemented: S_EVENT_MARK_ADDED, S_EVENT_MARK_CHANGE, S_EVENT_MARK_REMOVED
end

local eventHandler = {}
function eventHandler:onEvent(event)
    local ok, err = pcall(onEvent, event)
    if not ok then
        env.info("[JSONRPC] Error in event handler: "..tostring(err))
    end
end
world.addEventHandler(eventHandler)