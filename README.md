# NFP - Niki's File Protocol

NFP is a simple file transfer protocol

This repository provides a client and server.

## Installation

### Build from source:

Go into one of the two directories and type:
```
cargo build
```

The two binaries (server and client) can be compiled and placed into ```/usr/bin``` using:

```
./install.sh
```


## Usage

### Server
Start the server up with

```
./nfp_server
```

The server accepts a couple of flags

```-p``````--port``` - Specifies the port onto which the server binds (default 6969)
```-i``````--ip``` - Specifies the ip address onto which the server binds (default 6969)

```-d``````--directory``` - Specifies the working directory of the server. All of the requests to the server will use this directory as their base

```-s``````--safe``` - Specifies the safe directory. Requests cannot access files lower then this directory. By default, safe mode is disabled


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
