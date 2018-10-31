# Documentation

## Methods

### [`outText`](https://wiki.hoggitworld.com/view/DCS_func_outText)

Displays the passed string of text for the specified time to all players.

**Params:**
- *text* (string) - the message that should be displayed
- *displayTime* (number) - the amount of seconds the message should be displayed
- *clearView* (boolean) - defines whether or not to use the old message display format

### [`groupExists`](https://wiki.hoggitworld.com/view/DCS_func_isExist)

Return a boolean value based on whether the group currently exists in the mission.

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






