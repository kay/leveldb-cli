# leveldb-cli

A CLI for leveldb

# Installation

```
cargo install --git https://github.com/kay/leveldb-cli.git
```

# Quickstart

Create a new database on first run
```
# List all entries in a db at the path `example.db`
leveldb-cli example.db list
```
Insert some entries
```
# Put an entry in the db
leveldb-cli example.db put key1 value1
leveldb-cli example.db put key2 value2
```
```
leveldb-cli example.db list
```
Displays
```
key1=value1
key2=value2
```
Update an entry
```
# Just put it again
leveldb-cli example.db put key2 value2B
```
Read a specific entry
```
# Will print: key2=value2B
leveldb-cli example.db get key2
```
Delete a specific entry
```
leveldb-cli example.db delete key2
```
```
# prints: key2 is absent
leveldb-cli example.db get key2
```

# Docker

To run the latest docker image
```
docker run -it --rm ghcr.io/kay/leveldb-cli:main
```