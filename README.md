<!---
WhiteBeam Client

Transparent endpoint security

Copyright 2020, WhiteBeam Security, Inc.
--->

---

<br>
<p align="center">
<img src="https://raw.githubusercontent.com/gist/noproto/f858188c6201b9a7e4ac99157c2546ba/raw/f34a53aa2fc2ea6c3af8a26af43385719318640f/WhiteBeamShield.svg" alt="WhiteBeam Logo"/>
<br><br>
<img src="https://gist.githubusercontent.com/noproto/f858188c6201b9a7e4ac99157c2546ba/raw/37f3b631bbea096926d28cebdcee15654f6fe847/WhiteBeamTextOnly.svg" alt="WhiteBeam">
<br>
<i>Transparent endpoint security</i>
<br><br>
<a href="https://github.com/WhiteBeamSec/WhiteBeam/releases" title="Releases"><img src="https://img.shields.io/github/v/tag/WhiteBeamSec/WhiteBeam.svg?style=for-the-badge&label=release&color=blue" alt="Latest Release"></a>
<a href="https://github.com/WhiteBeamSec/WhiteBeam/security/policy" title="Security"><img src="https://img.shields.io/badge/bounty-$5,000-blue?style=for-the-badge" alt="Bounty $5,000"></a>
<a href="https://discord.gg/GYSVqYx" target="_blank" title="Chat"><img src="https://img.shields.io/discord/641744447289294859?style=for-the-badge" alt="Chat"></a>
</p>

---

## Features

* Block and detect advanced attacks
* Modern audited cryptography: [RustCrypto](https://github.com/RustCrypto) for hashing and encryption
* Highly compatible: Development focused on all platforms (incl. legacy) and architectures
* Source available: Audits welcome
* Reviewed by security researchers with combined 100+ years of experience

## In Action

* [Video demonstration of detection and prevention capabilities](https://whitebeamsec.com/)
* [Testing WhiteBeam against zeroday exploits](https://vs.whitebeamsec.com/)
* [Recorded attacks against the WhiteBeam honeypot](https://asciinema.org/~wbhoneypot)

## Installation

<details>
  <summary>WhiteBeam is currently unavailable for installation due to backwards-incompatible security enhancements for 0.3. Check back June 27th!</summary>

### From Packages (Linux)

Distro-specific packages have not been released yet for WhiteBeam, check again soon!

<!--- TODO: Using your package manager of choice (on Ubuntu/Debian (apt/snap classic)/Gentoo (emerge)/Arch (pacman AUR)/RHEL/Amazon Linux/Rocky Linux (yum)/OpenSUSE/etc.), details on installing `whitebeam` package. --->

### From Releases (Linux)

1. Download the [latest release](https://github.com/WhiteBeamSec/WhiteBeam/releases)
2. Ensure the release file hash matches the official hashes ([How-to](https://github.com/WhiteBeamSec/WhiteBeam/wiki/Verifying-file-hashes))
3. Install:
    * `./whitebeam-installer install`

### From Source (Linux)

1. Run tests (_Optional_):
    * `cargo run test`
2. Compile:
    * `cargo run build`
3. Install WhiteBeam:
    * `cargo run install`

</details><br/>

## Quick start
1. Become root (`sudo su`/`su root`)
2. Set a recovery secret: `whitebeam --setting RecoverySecret mask`. After setting the recovery secret, you can run `whitebeam --auth` to make changes to the system.

### How to Detect Attacks with WhiteBeam
Multiple guides are provided depending on your preference. [Contact us](mailto:info@whitebeamsec.com) so we can help you integrate WhiteBeam with your environment.
1. [Serverless guide](https://github.com/WhiteBeamSec/WhiteBeam/wiki), for passive review
2. [osquery Fleet setup guide](https://github.com/WhiteBeamSec/WhiteBeam/wiki), for passive review
3. [WhiteBeam Server setup guide](https://github.com/WhiteBeamSec/WhiteBeam/wiki), for active response

### How to Prevent Attacks with WhiteBeam

:information_source: WhiteBeam is <u>experimental</u> software.
[Contact us](mailto:info@whitebeamsec.com) for assistance safely implementing it.

1. Become root (`sudo su`/`su root`)
2. Review the baseline at least 24 hours after installing WhiteBeam:
    * `whitebeam --baseline`
3. Add trusted behavior to the whitelist, following the [whitelisting guide](https://github.com/WhiteBeamSec/WhiteBeam/wiki)
4. Enable WhiteBeam prevention:
    * `whitebeam --setting Prevention true`