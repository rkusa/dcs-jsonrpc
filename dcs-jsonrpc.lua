env.info("[JSONRPC] loading ...")

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

function groupByIdentifier(params)
    if type(params.name) == "string" then
        return Group.getByName(params.name)
    else
        return nil
    end
end

function unitByIdentifier(params)
    if type(params.name) == "string" then
        return Unit.getByName(params.name)
    else
        return nil
    end
end

function staticByIdentifier(params)
    if type(params.name) == "string" then
        return StaticObject.getByName(params.name)
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

    return success(nil)
end

function method_outTextForGroup(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params.group)
    if group == nil then
        return success(nil)
    end

    trigger.action.outTextForGroup(group:getID(), params.text, params.displayTime, params.clearView)

    return success(nil)
end

function method_removeMark(params)
    -- TODO: return error on missing params
    trigger.action.removeMark(params.id)

    return success(nil)
end

function method_getZone(params)
    -- TODO: return error on missing params
    local zone = trigger.misc.getZone(params.name)
    if zone == nil then
        return success(nil)
    else
        return success(zone)
    end
end

function method_getZones(params)
    -- TODO: return error on missing params
    local zones = {}
    for _, zone in pairs(env.mission.triggers.zones) do
        zones.insert(zones, zone.name)
    end
    return success(zones)
end

function method_getUserFlag(params)
    -- TODO: return error on missing params
    return success(trigger.misc.getUserFlag(params.flag))
end

function method_setUserFlag(params)
    -- TODO: return error on missing params
    trigger.action.setUserFlag(params.flag, params.value)
    return success(nil)
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

function method_groupID(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(nil)
    else
        return success(group:getID())
    end
end

function method_groupName(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return success(nil)
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

function starts_with(str, start)
    if type(str) ~= 'string' then
        return false
    end
    return str:sub(1, #start) == start
end

function dict_value(key)
    if starts_with(key, "DictKey_") then
        return env.getValueDictByKey(key)
    else
        return key
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

    local id = group:getID()
    for _, country in pairs(countries) do
        for kind, category in pairs(country) do
            if kind ~= 'static' then
                if type(category) == 'table' and type(category.group) == 'table' then
                    for _, groupData in pairs(category.group) do
                        if groupData.groupId == id then
                            groupData.name = dict_value(groupData.name)
                            groupData.category = Group.getByName(groupData.name):getCategory()
                            for _, unit in pairs(groupData.units) do
                                unit.name = dict_value(unit.name)
                            end
                            return success(groupData)
                        end
                    end
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

function method_groupUnits(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    local unitNames = {}
    if group ~= nil then
        for i, unit in pairs(group:getUnits()) do
            unitNames[i] = unit:getName()
        end
    end
    return success(unitNames)
end

function method_groupSmoke(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return error("Group does not exist")
    end

    group:markGroup(true)

    return success(nil)
end

function method_groupUnsmoke(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group == nil then
        return error("Group does not exist")
    end

    group:markGroup(false)

    return success(nil)
end

function method_groupDestory(params)
    -- TODO: return error on missing params
    local group = groupByIdentifier(params)
    if group ~= nil then
        group:destroy()
    end
    return success(nil)
end

--
-- RPC Unit methods
--

function method_unitName(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:getID())
    end
end

function method_unitExists(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(false)
    else
        return success(unit:isExist())
    end
end

function method_unitPosition(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:getPoint())
    end
end

function method_unitInfantryLoad(params)
    -- TODO: return error on missing params
    local load = groupByIdentifier(params.load)
    local into = unitByIdentifier(params.into)

    if load == nil then
        return error("Loaded group does not exist")
    end
    if into == nil then
        return error("Loading group does not exist")
    end

    load:embarking(into:getObjectID())

    return success(nil)
end

function method_unitInfantryCapacity(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:getDescentCapacity())
    end
end

function method_unitInfantryLoaded(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:getDescentOnBoard())
    end
end

function method_unitInfantryUnload(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params.unit)
    local group = groupByIdentifier(params.unload)

    if unit == nil then
        return error("Unit does not exist")
    end
    if group == nil then
        return error("Group does not exist")
    end

    unit:disembarking(group:getID())

    return success(nil)
end

function method_unitInfantrySmokeUnloadArea(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params.unit)
    local group = unitByIdentifier(params.smokeFor)

    if unit == nil then
        return error("Unit does not exist")
    end
    if group == nil then
        return error("Group does not exist")
    end

    unit:markDisembarkingTask(group:getID())

    return success(nil)
end

function method_unitLoadedGroups(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    local groupNames = {}
    if unit ~= nil then
        for i, group in pairs(coalition.getDescentsOnBoard(unit:getObjectID())) do
            groupNames[i] = group:getName()
        end
    end
    return success(groupNames)
end

function method_unitIsAirborne(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:inAir())
    end
end

function method_unitOrientation(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return success(nil)
    else
        return success(unit:getPosition())
    end
end

function method_unitGroup(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return error("Unit does not exist")
    end

    local group = unit:getGroup()
    if group == nil then
        return error("Group does not exist")
    end

    return success(group:getName())
end

function method_unitLife(params)
    -- TODO: return error on missing params
    local unit = unitByIdentifier(params)
    if unit == nil then
        return error("Unit does not exist")
    end

    return success(unit:getLife())
end


--
-- RPC Static methods
--

function method_addStatic(params)
    -- TODO: return error on missing params
    coalition.addStaticObject(params.country, params.data)
    -- Note: the static does not exist immediately, why we cannot do something like
    -- staticobj:getName() here
    return success(nil)
end

function method_staticID(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(nil)
    else
        return success(staticobj:getID())
    end
end

function method_staticName(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(nil)
    else
        return success(staticobj:getName())
    end
end

function method_staticExists(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(false)
    else
        return success(staticobj:isExist())
    end
end

function method_staticData(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(nil)
    end

    local countries = {}
    if staticobj:getCoalition() == coalition.side.RED then
        countries = env.mission.coalition.red.country
    else
        countries = env.mission.coalition.blue.country
    end

    local id = tonumber(staticobj:getID())
    env.info("[JSONRPC] SF: "..type(id))
    for _, country in pairs(countries) do
        for kind, category in pairs(country) do
            if kind == 'static' then
                if type(category) == 'table' and type(category.group) == 'table' then
                    for _, groupData in pairs(category.group) do
                        for _, unit in pairs(groupData.units) do
                            env.info("[JSONRPC] ID: "..type(unit.unitId).." "..tostring(unit.unitId == id))
                            if unit.unitId == id then
                                unit.name = dict_value(unit.name)
                                -- for statics return the first unit
                                return success(unit)
                            end
                        end
                    end
                end
            end
        end
    end

    return success(nil)
end

function method_staticPosition(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(nil)
    else
        return success(staticobj:getPoint())
    end
end

function method_staticCountry(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj == nil then
        return success(nil)
    else
        return success(staticobj:getCountry())
    end
end

function method_staticDestory(params)
    -- TODO: return error on missing params
    local staticobj = staticByIdentifier(params)
    if staticobj ~= nil then
        staticobj:destroy()
    end
    return success(nil)
end

--
-- RPC Mission Commands methods
--

function method_addSubMenu(params)
    -- TODO: return error on missing params
    local path = missionCommands.addSubMenu(params.name, params.path)
    return success(path)
end

function method_addGroupSubMenu(params)
    -- TODO: return error on missing params
    local path = missionCommands.addSubMenuForGroup(params.groupID, params.name, params.path)
    return success(path)
end

function method_addCoalitionSubMenu(params)
    -- TODO: return error on missing params
    local path = missionCommands.addSubMenuForCoalition(params.coalition, params.name, params.path)
    return success(path)
end

function method_addCommand(params)
    -- TODO: return error on missing params
    local path = missionCommands.addCommand(
        params.name,
        params.path,
        handleCommand,
        params.command
    )
    return success(path)
end

function method_addGroupCommand(params)
    -- TODO: return error on missing params
    local path = missionCommands.addCommandForGroup(
        params.groupID,
        params.name,
        params.path,
        handleCommand,
        params.command
    )
    return success(path)
end

function method_addCoalitionCommand(params)
    -- TODO: return error on missing params
    local path = missionCommands.addCommandForCoalition(
        params.coalition,
        params.name,
        params.path,
        handleCommand,
        params.command
    )
    return success(path)
end

function method_removeEntry(params)
    -- TODO: return error on missing params
    missionCommands.removeItem(params.path)
    return success("ok")
end

function method_removeGroupEntry(params)
    -- TODO: return error on missing params
    missionCommands.removeItemForGroup(params.groupID, params.path)
    return success("ok")
end

function method_removeCoalitionEntry(params)
    -- TODO: return error on missing params
    missionCommands.removeItemForCoalition(params.coalition, params.path)
    return success("ok")
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

function handleCommand(command)
    jsonrpc.broadcast("CommandSelect", json:encode({
        time = timer.getTime(),
        command = command,
    }))
end

--
-- RPC request handler
--
function handleRequest(method, params)
    --env.info("[JSON-RPC] receiving method "..method.." with params: "..tostring(params))

    local fnName = "method_"..method
    local fn = _G[fnName]
    if params ~= nil then
        params = json:decode(params)
    end

    if type(fn) == "function" then
        local ok, result = pcall(fn, params)
        if not ok then
            env.error("[JSON-RPC] error executing "..method.." with params: "..tostring(params)..": "..tostring(result))
        end

        return result
    else
        return {
            error = "unsupported method "..method
        }
    end
end

--
-- execute JSON-RPC requests every 0.02 seconds
--
timer.scheduleFunction(function(arg, time)
    while jsonrpc.next(handleRequest) do
        -- TODO: restrict handled requests per tick?
    end

    return timer.getTime() + .02 -- return time of next call
end, nil, timer.getTime() + .02)

--
-- listen to DCS events
--
function identifier(obj)
    if obj == nil then
        return nil
    end
    return obj:getName()
end

function onEvent(event)
    
    if event.id == world.event.S_EVENT_SHOT then
        jsonrpc.broadcast("Shot", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
            weapon = { id = event.weapon:getName() },
        }))

    elseif event.id == world.event.S_EVENT_HIT then
        local target ={
            id = tonumber(event.target:getID()),
            name = event.target:getName() or "",
            category = event.target:getCategory(),
        }
        jsonrpc.broadcast("Hit", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
            weapon = { id = event.weapon:getName() },
            target = target,
        }))

    elseif event.id == world.event.S_EVENT_TAKEOFF then
        jsonrpc.broadcast("Takeoff", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
            place = identifier(event.place),
        }))

    elseif event.id == world.event.S_EVENT_LAND then
        jsonrpc.broadcast("Land", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
            place = identifier(event.place),
        }))

    elseif event.id == world.event.S_EVENT_CRASH then
        jsonrpc.broadcast("Crash", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_EJECTION then
        jsonrpc.broadcast("Ejection", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_REFUELING then
        jsonrpc.broadcast("Refueling", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_DEAD then
        jsonrpc.broadcast("Dead", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_PILOT_DEAD then
        jsonrpc.broadcast("PilotDead", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_BASE_CAPTURED then
        jsonrpc.broadcast("BaseCapture", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
            place = identifier(event.place),
        }))

    elseif event.id == world.event.S_EVENT_MISSION_START then
        jsonrpc.broadcast("MissionStart", json:encode({
            time = event.time,
        }))

    elseif event.id == world.event.S_EVENT_MISSION_END then
        jsonrpc.broadcast("MissionEnd", json:encode({
            time = event.time,
        }))
        jsonrpc.stop()

    -- unimplemented: S_EVENT_TOOK_CONTROL

    elseif event.id == world.event.S_EVENT_REFUELING_STOP then
        jsonrpc.broadcast("RefuelingStop", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_BIRTH then
        jsonrpc.broadcast("Birth", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_HUMAN_FAILURE then
        jsonrpc.broadcast("SystemFailure", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_ENGINE_STARTUP then
        jsonrpc.broadcast("EngineStartup", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_ENGINE_SHUTDOWN  then
        jsonrpc.broadcast("EngineShutdown", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_PLAYER_ENTER_UNIT then
        jsonrpc.broadcast("PlayerEnterUnit", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_PLAYER_LEAVE_UNIT then
        jsonrpc.broadcast("PlayerLeaveUnit", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    -- unimplemented: S_EVENT_PLAYER_COMMENT

    elseif event.id == world.event.S_EVENT_SHOOTING_START then
        jsonrpc.broadcast("ShootingStart", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_SHOOTING_END then
        jsonrpc.broadcast("ShootingEnd", json:encode({
            time = event.time,
            initiator = identifier(event.initiator),
        }))

    elseif event.id == world.event.S_EVENT_MARK_ADDED then
        jsonrpc.broadcast("MarkAdd", json:encode({
            time = event.time,
            groupId = event.groupID > -1 and event.groupID or nil,
            coalition = event.coalition > -1 and event.coalition or nil,
            id = event.idx,
            initiator = identifier(event.initiator),
            -- x and z are rotated here compared to group/unit coords
            pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
            text = event.text,
            -- ignored: id, groupID
        }))

    elseif event.id == world.event.S_EVENT_MARK_CHANGE then
        jsonrpc.broadcast("MarkChange", json:encode({
            time = event.time,
            groupId = event.groupID > -1 and event.groupID or nil,
            coalition = event.coalition > -1 and event.coalition or nil,
            id = event.idx,
            initiator = identifier(event.initiator),
            -- x and z are rotated here compared to group/unit coords
            pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
            text = event.text,
            -- ignored: id, groupID
        }))

    elseif event.id == world.event.S_EVENT_MARK_REMOVED then
        jsonrpc.broadcast("MarkRemove", json:encode({
            time = event.time,
            groupId = event.groupID > -1 and event.groupID or nil,
            coalition = event.coalition > -1 and event.coalition or nil,
            id = event.idx,
            initiator = identifier(event.initiator),
            -- x and z are rotated here compared to group/unit coords
            pos = { x = event.pos.z, y = event.pos.y, z = event.pos.x },
            text = event.text,
            -- ignored: id, groupID
        }))

    end

    --env.info("[JSONRPC] Event: "..inspect(event))
end

local eventHandler = {}
function eventHandler:onEvent(event)
    local ok, err = pcall(onEvent, event)
    if not ok then
        env.error("[JSONRPC] Error in event handler: "..tostring(err))
    end
end
world.addEventHandler(eventHandler)

env.info("[JSONRPC] loaded ...")