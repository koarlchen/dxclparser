# libdxclparser

A regex-based parser for the telnet interface provided by a DX cluster.

See `example/` folder for exemplary usage.


## Supported DX-Clusters

* DXSpider
* AR-Cluster
* CC Cluster

## Supported types of Spots

* DX
* RBN
* WCY
* WWV
* WX
* ToAll
* ToLocal

## Build, Test and Run

To build this library, simply execute `cargo build [--release]`.

The basic test cases can be executed through `cargo test`.

The example `basic.rs` parses a spot given as a commandline argument. See therefore also the shell script `basic_run.sh` which uses netcat to connect to a cluster server and outputs the parsed spot in its json format.
The example `type.rs` also takes a spot as a commandline argument but demonstrates how to handle each type of spot separately.
The example `file.rs` reads a file given as a commandline argument line by line and outputs the parsed spots in its json format.


## Parser

A given spot will be parsed by the corresponding regular expression. Since the different cluster software implementations format the spot slightly different, some fields may be missing and are marked as optional.