**How do I report a vulnerability to WhiteBeam?**

Please email us at security@whitebeamsec.com. We request at least the following information:

* A short summary of the potential impact of the issue (if known).
* Details explaining how to reproduce the issue or how an exploit may be formed.
* Your name (optional). If provided, we will provide credit for disclosure. Otherwise, you will be treated anonymously and your privacy will be respected.
* Your email or other means of contacting you.
* A PGP key/fingerprint for us to provide encrypted responses to your disclosure. If this is not provided, we cannot guarantee that you will receive a response prior to a fix being made and deployed

**Important**: Use our GPG key to encrypt this information (saved as a plain text file attached to your email):

gpg --keyserver hkp://pgp.mit.edu:80 --recv-keys 4A3F1233C01563F808B8355125ECFD172151528B

**Current security vulnerability rewards**

Rewards will be provided at the discretion of the lead developers. All vulnerabilities must be demonstrated in the challenge environment to be eligible for payment.

| Vulnerability                                                                               | Reward         |
| ------------------------------------------------------------------------------------------- | -------------- |
| Remote code execution (RCE)                                                                 | $5000, Credits |
| Local privilege escalation (LPE)                                                            | $2000, Credits |
| Bypass whitelisting<sup>\*</sup> ([Try the challenge!](https://challenge.whitebeamsec.com)) | $1000, Credits |
| Cryptographic vulnerability                                                                 | $250, Credits  |
| WhiteBeam service crash (DoS)                                                               | $50            |

<sup>\* Must be a program presently whitelisted by WhiteBeam Security, Inc. exhibiting documented behavior or a common OS kernel/dynamic linker feature that bypasses WhiteBeam. Please report vulnerabilities in third party software to their respective vendors.</sup>

Past security advisories can be found here: https://github.com/WhiteBeamSec/WhiteBeam/security/advisories

We would like to thank the following security researchers for their contributions to WhiteBeam's security:

| Researchers          | Date        | :trophy:           |
| -------------------- | ----------- | ------------------ |
| *gemini*, *brianx*   | Nov 6, 2019 | [WhiteBeam 0.0.5](https://github.com/WhiteBeamSec/WhiteBeam/security/advisories/GHSA-mm3f-f5hg-p2hv)  |
