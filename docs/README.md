# Documentation

## Methods

### [`outText`](https://wiki.hoggitworld.com/view/DCS_func_outText)

Displays the passed string of text for the specified time to all players.

**Params:**
- *text* (string) - the message that should be displayed
- *displayTime* (number) - the amount of seconds the message should be displayed
- *clearView* (boolean) - defines whether or not to use the old message display format

## Events

Subscribe to events by calling the `subscribe` method and providing the event name as a `name` parameter, e.g.:

```json
>> {"jsonrpc":"2.0","method":"subscribe","params":{"name":"player_enter_unit"},"id":2}
```

### `player_enter_unit`

Occurs when any player assumes direct control of a unit.

**Params:**
- *initiator* - The name of the unit that is being taken control of






