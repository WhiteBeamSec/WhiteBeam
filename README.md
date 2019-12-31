![WhiteBeam](https://gist.githubusercontent.com/noproto/ea7d62cd578afdd1bac2e96078c0e6b2/raw/cf895a5fef1f2295671653ece9155f4e1f0478e4/WhiteBeam.svg?sanitize=true)

WhiteBeam is an OSS EDR application with cross platform application whitelisting. While it is in pre-release development, it should not be used in production environments.

# Getting started

## Installation

### Binaries

**Important**: Always ensure the downloaded file hash matches official hashes ([How-to](https://github.com/noproto/WhiteBeam/wiki/Verifying-file-hashes)).

| Platform       | URL                                                                | Hash(es) |
| -------------- | ------------------------------------------------------------------ | -------- |
| Linux (64-bit) | https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.tar.gz | [SHA-256](https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.SHA256) |

Install WhiteBeam: `./install.sh`

### Source (Linux)

1. (Optional) Run tests:
`make test`
2. Compile:
`make`
3. Install WhiteBeam:
`make install`

## Configuring

1. Add permitted applications:
`whitebeam --add /absolute/path/to/command`
2. Enable WhiteBeam:
`whitebeam --enable`

# In Action

[![asciicast](https://asciinema.org/a/291097.svg)](https://asciinema.org/a/291097)
