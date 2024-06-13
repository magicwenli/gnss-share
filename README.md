# gnss-share

[![GitHub Release](https://img.shields.io/github/v/release/magicwenli/gnss-share)](https://github.com/magicwenli/gnss-share/releases)
[![GitHub Actions Workflow Status](https://github.com/magicwenli/gnss-share/actions/workflows/release.yaml/badge.svg)](https://github.com/magicwenli/gnss-share/actions/workflows/release.yaml)


Utility to share your GPS device on local network.


## Goals

This repo is a fork of the original project [gps-share](https://github.com/zeenix/gps-share)

I forked it to add support for additional features and to make it more generic.

- support for tcp/unix clients send commands to gnss serial.
- update some crates
- use mio/mio-serial instead of serial

## Command-line usage

```plaintext
Utility to share your GNSS device on local network.

Usage: gnss-share [OPTIONS] <DEVICE>

Arguments:
  <DEVICE>  GNSS device node

Options:
  -b, --baudrate <BAUDRATE>        GNSS device baudrate [default: 115200]
  -i, --interface <INTERFACE>      TCP service IP or net iface. Default is binding all iface
  -p, --port <PORT>                TCP service port [default: 10110]
  -n, --no-tcp                     Disable TCP service
  -s, --socket-path <SOCKET_PATH>  Unix socket service path. Default is disable
  -d, --daemonize                  Daemonize the process
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```