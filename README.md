![WhiteBeam](https://gist.githubusercontent.com/noproto/ea7d62cd578afdd1bac2e96078c0e6b2/raw/cf895a5fef1f2295671653ece9155f4e1f0478e4/WhiteBeam.svg?sanitize=true)

WhiteBeam is an OSS EDR application with cross platform application whitelisting, written in Rust. While it is in pre-release development, it should not be used in production environments.

# Getting started

## Binaries
Release binaries will be provided as of WhiteBeam 0.1.0.

## Compile (Linux)
Update src/library/common/whitelist.rs to reflect your whitelist. Dynamic whitelists and baselines will be available by November 1st, 2019.

1. Compile:
`make`
2. Install WhiteBeam:
`make install`

## Tests (Linux)
`make test`

# In Action

[![asciicast](https://asciinema.org/a/269329.svg)](https://asciinema.org/a/269329)
