# NFP - Niki's File Protocol

NFP is a simple file transfer protocol

This repository provides a client and server.

## Installation
---

### Build from source:

Go into one of the two directories and type:
```
cargo build
```


## Usage
---

### Server
Start the server up with

```
./nfp_server
```

### Client
Example usage:

```
./nfp_client 127.0.0.1 ls
```

The first argument is the address of the NFP server. You can optionally provide the port with the ```-p``` flag.

The second argument is the command. The command is one of these:

```
cat
echo
ls
rm
rmr
cp
```

```cat``` - Reades the file (first argument) from the server to the stdout

```echo``` - Echos the provided text (second argument) to the file (first argument) on the server

```ls``` - Like ```ls```

```rm``` - Like ```rm```

```rmr``` - Like ```rm```, but with the ```-r``` flag

```cp``` - Copies a file from the client (first argument), to the given file on the server (second argument)
