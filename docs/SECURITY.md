**How do I report a vulnerability to WhiteBeam?**

For now the project is small enough that you can just submit an issue to our issue tracker. This will be changed in the future.

Current security vulnerability rewards (will be provided at the discretion of the lead developers):

| Vulnerability       | Reward        |
| ------------------- | ------------- |
| Bypass whitelisting | $100, Credits |

---

We would like to thank the following security researchers for their contributions to WhiteBeam's security:

| Version | Researcher(s)  | Description | Fix available |
| ------- | -------------- | ----------- | ------------- |
| 0.0.5   | gemini, brianx | If LD_PRELOAD or LD_AUDIT is defined to a nonexecutable shared object library, execution of non-whitelisted library functions is possible | :heavy_check_mark: (0.0.6) |
