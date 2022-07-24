# leveldb-cli

A CLI for leveldb

# Quickstart

Create a new database on first run
```
# List all entries in a db at the path `example.db`
cargo run -q -- example.db list
```
Insert some entries
```
# Put an entry in the db
cargo run -q -- example.db put key1 value1
cargo run -q -- example.db put key2 value2
```
```
cargo run -q -- example.db list
```
Displays
```
key1=value1
key2=value2
```
Update an entry
```
# Just put it again
cargo run -q -- example.db put key2 value2B
```
Read a specific entry
```
# Will print: key2=value2B
cargo run -q -- example.db get key2
```
Delete a specific entry
```
cargo run -q -- example.db delete key2
```
```
# prints: key2 is absent
cargo run -q -- example.db get key2
```
