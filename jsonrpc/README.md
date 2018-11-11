# DCS JSON-RPC

## Methods

#### `execute`

Execute a given Lua code.

**Params:**
- *lua* (string) - the Lua code that should be executed

**Example:**

```json
>> {"jsonrpc":"2.0","method":"execute","params":{"lua":"return 40 + 2"},"id":1}
<< {"jsonrpc":"2.0","result":"42","id":1}
```

### Trigger Methods

#### [`outText`](https://wiki.hoggitworld.com/view/DCS_func_outText)

Displays the passed string of text for the specified time to all players.

**Params:**
- *text* (string) - the message that should be displayed
- *displayTime* (number) - the amount of seconds the message should be displayed
- *clearView* (boolean) - defines whether or not to use the old message display format

#### [`removeMark`](https://wiki.hoggitworld.com/view/DCS_func_removeMark)

Removes a mark panel from the f10 map

**Params:**
- *id* (number) - the id of the mark that should be removed

### Mission Command Methods

#### [`addSubMenu`](https://wiki.hoggitworld.com/view/DCS_func_addSubMenu)

Creates a F10 submenu for all players.

**Params:**
- *name* (string) - the menu name displayed in the F10 menu
- [*path*] (table) - the path where the submenu should be added to

#### [`addGroupSubMenu`](https://wiki.hoggitworld.com/view/DCS_func_addSubMenu)

Creates a F10 submenu for all players of the specified group.

**Params:**
- *groupID* (int) - the id of the group
- *name* (string) - the menu name displayed in the F10 menu
- [*path*] (table) - the path where the submenu should be added to

#### [`addCoalitionSubMenu`](https://wiki.hoggitworld.com/view/DCS_func_addSubMenuForCoalition)

Creates a F10 submenu for all players of the specified coalition.

**Params:**
- *coalition* (u8) - the coalition
- *name* (string) - the menu name displayed in the F10 menu
- [*path*] (table) - the path where the submenu should be added to

#### [`addCommand`](https://wiki.hoggitworld.com/view/DCS_func_addCommand)

Adds a command for all players.

**Params:**
- *name* (string) - the command name displayed in the F10 menu
- [*path*] (table) - the path where the command should be added at
- command (table) - the command (this value will be included in the event when the command is selected by a player)

#### [`addGroupCommand`](https://wiki.hoggitworld.com/view/DCS_func_addCommandForGroup)

Adds a command for all players of the specified group.

**Params:**
- *groupID* (int) - the id of the group
- *name* (string) - the command name displayed in the F10 menu
- [*path*] (table) - the path where the command should be added at
- command (table) - the command (this value will be included in the event when the command is selected by a player)

#### [`addCoalitionCommand`](https://wiki.hoggitworld.com/view/DCS_func_addCommandForCoalition)

Adds a command for all players of the specified coalition.

**Params:**
- *coalition* (u8) - the coalition
- *name* (string) - the command name displayed in the F10 menu
- [*path*] (table) - the path where the command should be added at
- command (table) - the command (this value will be included in the event when the command is selected by a player)

#### [`removeEntry`](https://wiki.hoggitworld.com/view/DCS_func_removeItem)

Removes the submenu (and its children) or command from the F10 menu for all players.

**Params:**
- [*path*] (table) - the path of the submenu that should be removed

#### [`removeGroupEntry`](https://wiki.hoggitworld.com/view/DCS_func_removeItemForGroup)

Removes the submenu (and its children) or command from the F10 menu for all players of the specified group.

**Params:**
- *groupID* (int) - the id of the group
- [*path*] (table) - the path of the submenu that should be removed

#### [`removeCoalitionEntry`](https://wiki.hoggitworld.com/view/DCS_func_removeItemForCoalition)

Removes the submenu (and its children) or command from the F10 menu for all players of the specified coalition.

**Params:**
- *coalition* (u8) - the coalition
- [*path*] (table) - the path of the submenu that should be removed

### Group Methods

#### [`getGroups`](https://wiki.hoggitworld.com/view/DCS_func_getGroups)

Get a list of all group names of the given coalition and category.

**Params:**
- *coalition* (u8) - the coalition
- [*category*] (u8) - the group category

#### [`groupID`](https://wiki.hoggitworld.com/view/DCS_func_getID)

Returns the group's id.

**Params:**
- *name* (string) - the name of the group

#### [`groupName`](https://wiki.hoggitworld.com/view/DCS_func_getName)

Returns the group's name.

**Params:**
- *id* (int) - the id of the group

#### [`groupExists`](https://wiki.hoggitworld.com/view/DCS_func_isExist)

Return a boolean value based on whether the group currently exists in the mission.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

#### [`groupCoalition`](https://wiki.hoggitworld.com/view/DCS_func_getCoalition)

Returns the group's coalition.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

#### [`groupCountry`](https://wiki.hoggitworld.com/view/DCS_func_getCountry)

Returns the group's country.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

#### [`groupCategory`](https://wiki.hoggitworld.com/view/DCS_func_getCategory)

Returns the group's category.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

### `groupData`

Returns the group data as defined in the mission editor. Result might be null, if the group was added later (and thus not defined in the mission editor).

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

#### [`addGroup`](https://wiki.hoggitworld.com/view/DCS_func_addGroup)

Dynamically spawns a group of the specified category for the specified country. Group data table is in the same format as created by the mission editor. Returns the name of the newly created group.

**Params:**
- *country* (u8) - the group's country
- *category* (u8) - the group's category
- *data* (u8) - the group data (same format as created by the mission editor)

#### [`groupActivate`](https://wiki.hoggitworld.com/view/DCS_func_activate)

Activates the group if the group has a delayed start or late activation.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

#### [`groupUnits`](https://wiki.hoggitworld.com/view/DCS_func_getUnits)

Get a list of all unit names that are part of the given group.

**Params:**
- *id* (int) or *name* (string) - the id or name of the group

### Unit Methods

#### [`unitName`](https://wiki.hoggitworld.com/view/DCS_func_getName)

Returns the group's name.

**Params:**
- *id* (int) - the id of the group

#### [`unitPosition`](https://wiki.hoggitworld.com/view/DCS_func_getPoint)

Returns the unit's x, y, z position relative to the map's origin.

**Params:**
- *id* (int) or *name* (string) - the id or name of the unit

## Events

Subscribe to events by calling the `subscribe` method and providing the event name as a `name` parameter, e.g.:

```json
>> {"jsonrpc":"2.0","method":"subscribe","params":{"name":"player_enter_unit"},"id":2}
```

### [`Shot`](https://wiki.hoggitworld.com/view/DCS_event_shot)

Occurs whenever any unit in a mission fires a weapon. But not any machine gun or autocannon based weapon, those are handled by shooting_start.

**Params:**
- _time_: the event's mission time
- _initiator_: The name of the unit that fired the weapon
- _weapon_: The name of the weapon that has been fired

### [`Hit`](https://wiki.hoggitworld.com/view/DCS_event_hit)

Occurs whenever an object is hit by a weapon.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit object the fired the weapon
- _weapon_: Weapon object that hit the target
- _target_: The Object that was hit.

### [`Takeoff`](https://wiki.hoggitworld.com/view/DCS_event_takeoff)

Occurs when an aircraft takes off from an airbase, farp, or ship.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that took off
- _place_: Object from where the AI took-off from. Can be an Airbase Object, FARP, or Ships

### [`Land`](https://wiki.hoggitworld.com/view/DCS_event_land)

Occurs when an aircraft lands at an airbase, farp or ship

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that has landed
- _place_: Object that the unit landed on. Can be an Airbase Object, FARP, or Ships

### [`Crash`](https://wiki.hoggitworld.com/view/DCS_event_crash)

Occurs when any aircraft crashes into the ground and is completely destroyed.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that has crashed

### [`Ejection`](https://wiki.hoggitworld.com/view/DCS_event_ejection)

Occurs when a pilot ejects from an aircraft

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that has ejected

### [`Refueling`](https://wiki.hoggitworld.com/view/DCS_event_refueling)

Occurs when an aircraft connects with a tanker and begins taking on fuel.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that is receiving fuel.

### [`RefuelingStop`](https://wiki.hoggitworld.com/view/DCS_event_refueling_stop)

Occurs when an aircraft is finished taking fuel.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that was receiving fuel.

### [`Dead`](https://wiki.hoggitworld.com/view/DCS_event_dead)

Occurs when an object is completely destroyed.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that is was destroyed.

### [`PilotDead`](https://wiki.hoggitworld.com/view/DCS_event_pilot_dead)

Occurs when the pilot of an aircraft is killed.

**Params:**
- _time_: the event's mission time
- _can_ occur either if the player is alive and crashes or if a weapon kills the pilot without completely destroying the plane.
- _initiator_: The unit that the pilot has died in.

### [`BaseCapture`](https://wiki.hoggitworld.com/view/DCS_event_base_captured)

Occurs when a ground unit captures either an airbase or a farp.

**Params:**
- _time_: the event's mission time
- _initiator_ : The unit that captured the base
- _place_: The airbase that was captured, can be a FARP or Airbase. When calling place:getCoalition() the faction will already be the new owning faction.

### [`MissionStart`](https://wiki.hoggitworld.com/view/DCS_event_mission_start)

Occurs when a mission starts

**Params:**
- _time_: the event's mission time

### [`MissionEnd`](https://wiki.hoggitworld.com/view/DCS_event_mission_end)

Occurs when a mission ends.

**Params:**
- _time_: the event's mission time

### [`Birth`](https://wiki.hoggitworld.com/view/DCS_event_birth)

Occurs when any object is spawned into the mission.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that was spawned

### [`SystemFailure`](https://wiki.hoggitworld.com/view/DCS_event_human_failure)

Occurs when any system fails on a human controlled aircraft.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that had the failure

### [`EngineStartup`](https://wiki.hoggitworld.com/view/DCS_event_engine_startup)

Occurs when any aircraft starts its engines.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that is starting its engines.

### [`EngineShutdown`](https://wiki.hoggitworld.com/view/DCS_event_engine_shutdown)

Occurs when any aircraft shuts down its engines.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that is stopping its engines

### [`PlayerEnterUnit`](https://wiki.hoggitworld.com/view/DCS_event_player_enter_unit)

Occurs when any player assumes direct control of a unit.

**Params:**
- _time_: the event's mission time
- _initiator_: The name of the unit that is being taken control of

### [`PlayerLeaveUnit`](https://wiki.hoggitworld.com/view/DCS_event_player_leave_unit)

Occurs when any player relieves control of a unit to the AI.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that the player left.

### [`ShootingStart`](https://wiki.hoggitworld.com/view/DCS_event_shooting_start)

Occurs when any unit begins firing a weapon that has a high rate of fire. Most common with aircraft cannons (GAU-8), autocannons, and machine guns.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that is doing the shooting
- _target_: The unit that is being targeted.

### [`ShootingEnd`](https://wiki.hoggitworld.com/view/DCS_event_shooting_end)

Occurs when any unit stops firing its weapon. Event will always correspond with a shooting start event.

**Params:**
- _time_: the event's mission time
- _initiator_: The unit that was doing the shooing.

### [`MarkAdd`](https://wiki.hoggitworld.com/view/DCS_event_mark_added)

Occurs when mark panels get added to the mission by players or scripting functions.

**Params:**
TODO

### [`MarkChange`](https://wiki.hoggitworld.com/view/DCS_event_mark_change)

Occurs when a mark panel is modified by a player.

**Params:**
TODO

### [`MarkRemove`](https://wiki.hoggitworld.com/view/DCS_event_mark_remove)

Occurs when mark panels get removed from the mission by players or scripting functions.

**Params:**
TODO

### `CommandSelect`

Occurs when a player selects an F10-menu command.

**Params:**
- _time_: the event's mission time
- _command_: the command that has been selected (this is the same data that has been added as `command` by `addCommand`RPC methods)




