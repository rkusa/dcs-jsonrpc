# DCS JSON-RPC

## Methods

### [`outText`](https://wiki.hoggitworld.com/view/DCS_func_outText)

Displays the passed string of text for the specified time to all players.

**Params:**
- *text* (string) - the message that should be displayed
- *displayTime* (number) - the amount of seconds the message should be displayed
- *clearView* (boolean) - defines whether or not to use the old message display format

### `execute`

Execute a given Lua code.

**Params:**
- *lua* (string) - the Lua code that should be executed

**Example:**

```json
>> {"jsonrpc":"2.0","method":"execute","params":{"lua":"return 40 + 2"},"id":1}
<< {"jsonrpc":"2.0","result":"42","id":1}
```

### [`getGroups`](https://wiki.hoggitworld.com/view/DCS_func_getGroups)

Get a list of all group names of the given coalition and category.

**Params:**
- *coalition* (u8) - the coalition
- [*category*] (u8) - the group category

### [`groupExists`](https://wiki.hoggitworld.com/view/DCS_func_isExist)

Return a boolean value based on whether the group currently exists in the mission.

**Params:**
- *name* (string) - the name of the group

### [`groupCoalition`](https://wiki.hoggitworld.com/view/DCS_func_getCoalition)

Returns the group's coalition.

**Params:**
- *name* (string) - the name of the group

### [`groupCountry`](https://wiki.hoggitworld.com/view/DCS_func_getCountry)

Returns the group's country.

**Params:**
- *name* (string) - the name of the group

### [`groupCategory`](https://wiki.hoggitworld.com/view/DCS_func_getCategory)

Returns the group's category.

**Params:**
- *name* (string) - the name of the group

### `groupData`

Returns the group data as defined in the mission editor. Result might be null, if the group was added later (and thus not defined in the mission editor).

**Params:**
- *name* (string) - the name of the group

### [`addGroup`](https://wiki.hoggitworld.com/view/DCS_func_addGroup)

Dynamically spawns a group of the specified category for the specified country. Group data table is in the same format as created by the mission editor. Returns the name of the newly created group.

**Params:**
- *country* (u8) - the group's country
- *category* (u8) - the group's category
- *data* (u8) - the group data (same format as created by the mission editor)

### [`groupActivate`](https://wiki.hoggitworld.com/view/DCS_func_activate)

Activates the group if the group has a delayed start or late activation.

**Params:**
- *name* (string) - the name of the group

## Events

Subscribe to events by calling the `subscribe` method and providing the event name as a `name` parameter, e.g.:

```json
>> {"jsonrpc":"2.0","method":"subscribe","params":{"name":"player_enter_unit"},"id":2}
```

### [`shot`](https://wiki.hoggitworld.com/view/DCS_event_shot)

Occurs whenever any unit in a mission fires a weapon. But not any machine gun or autocannon based weapon, those are handled by shooting_start.

**Params:**
- *time* - the event's mission time
- *initiator* - The name of the unit that fired the weapon
- *weapon* - The name of the weapon that has been fired

### [`mission_end`](https://wiki.hoggitworld.com/view/DCS_event_mission_end)

Occurs when a mission ends.

**Params:**
- *time* - the event's mission time

### [`player_enter_unit`](https://wiki.hoggitworld.com/view/DCS_event_player_enter_unit)

Occurs when any player assumes direct control of a unit.

**Params:**
- *time* - the event's mission time
- *initiator* - The name of the unit that is being taken control of






