# Overview

WhiteBeam is a cross platform, slim, and secure open source EDR solution that emphasizes application whitelisting. Core principles of our community-based solution are:

1. **Given enough eyes, all bugs are shallow**:
Our goal is not to obscure how the WhiteBeam client works through closed source, nor will we hide its configuration. We have discovered multiple vulnerabilities in leading closed-source EDR software that bypass enforcement and/or permit local privilege escalation through the EDR client itself.
We encourage hacking WhiteBeam. We plan on setting up a challenge server to bypass it, alongside our own efforts and a public bounty program (planned for version 1). Any disclosures prior to version 1 will be appreciated and listed in CREDITS.md.

2. **WhiteBeam is free**:
The WhiteBeam client is free and works with local configuration.
We do plan on creating a (separate) enterprise management server and offering support plans for WhiteBeam in the future to fund development efforts.

3. **Dynamic library loading is better than LKM**:
Leading EDR software commonly uses loadable kernel modules, and frequently isn't able to block malicious binaries from running.
WhiteBeam can be compiled once and used across many different platforms. It maintains the speed and security of LKM software, yet does not require a process to be running on the system.
