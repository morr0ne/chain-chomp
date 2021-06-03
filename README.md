# chain-chomp

## Rest-like api for the bitcoin rpc.

This is more a proof of concept than anything actually usefull.

The idea it to have a rocket server that wraps all the rpc functions

A the moment I'm using a brute force approach but in the future it would be useful to have a macro that automatically generates everything

usage:

```bash
chain-chomp --rpc-address <address> --rpc-user <user> --rpc-password <password>
```