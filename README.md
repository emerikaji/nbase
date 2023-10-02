# nBase

nBase is a simple key-value store, compatible with any data type- Although at the moment the data is converted to a string when obtained. Future implementations will include more possible return types.

## Technologies

nBase is written in rust and the files are written and compressed with the [minilzo-rs](https://crates.io/crates/minilzo-rs) crate.

## Syntax reference (will be included in a help command with next release):

`nbase usage_case name [command] [option | key | key value...]`

### usage_case
- `init`: Create a new `name`.ndb file
- `inits`: Create a new `name`.ndbs file
- `with`: Use `name`.ndb for a `command`
- `withs`: Use `name`.ndbs for a `command`

### command (.ndb)
- `push`: Add the `key` `value` pair
- `pushmany`: Add all following `key` `value` pairs
- `pull`: Get the value associated with `key`
- `edit`: Edit `key` to be associated with `value`
- `pop`: Remove the pair with key `key`
- `json`: Get a json file of the store

### command (.ndbs)
- `add`: Add the `option`.ndb file to the `name`.ndbs group
-  `json`: Get a json file of the group

## Roadmap
- Help command and full handling of any arguments
- Remove from group
- Data type support (inserting json arrays is possible, but it isn't considered a true array and as such isn't language-agnostic)
- Create python and golang libraries
- Stop usage of ";" at the program level
- Make all writing instructions atomic
