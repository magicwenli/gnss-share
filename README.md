# gnss-share

![GitHub Release](https://img.shields.io/github/v/release/magicwenli/gnss-share)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/magicwenli/gnss-share/release.yaml)


Utility to share your GPS device on local network.


## Goals

This repo is a fork of the original project [gps-share](https://github.com/zeenix/gps-share)

I forked it to add support for additional features and to make it more generic.

- support for tcp/unix clients send commands to gnss serial.
- update some crates
- use mio-server instead of serial

## Command-line usage

```plaintext
Utility to share your GNSS device on local network.

Usage: gnss-share [OPTIONS] <DEVICE>

Arguments:
  <DEVICE>  GPS device node

Options:
  -b, --baudrate <BAUDRATE>        Baudrate to use for communication with GPS device [default: 115200]
  -p, --port <PORT>                Port to run TCP service on [default: 10110]
  -i, --interface <INTERFACE>      Bind specific network interface
  -n, --no-tcp                     Don't share over TCP
  -s, --socket-path <SOCKET_PATH>  Path to place the unix socket service
  -h, --help                       Print help (see more with '--help')
```