# DCS JSON-RPC REPL

This is a simple repl that reads from stdin, executes the input within the DCS mission environment and returns the result.

Example

```
$ cargo run
return Group.getByName("group1")
= "null"
```
