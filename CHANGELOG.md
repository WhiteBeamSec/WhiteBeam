# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Improved baselines

### Fixed

- Linux LD_PRELOAD/LD_AUDIT library: Poisoned mutexes in multithreaded programs

## [0.2.2] - 2021-05-12

### Added

- Linux LD_PRELOAD/LD_AUDIT library: Support for 10 hooks

### Security
- The fopen/fopen64/truncate hooks (included in the Essential whitelist) allowed a file to be truncated in the OpenFileDescriptor action prior to the VerifyCanWrite action. This allowed arbitrary files to be truncated with sufficient privileges on Linux, including WhiteBeam startup files.
  Further, the FORTIFY_SOURCE variants of libc functions, truncate64, and ftruncate64 may have allowed similar bypasses to be possible.
  Fixed in 0.2.2: https://github.com/WhiteBeamSec/WhiteBeam/security/advisories/GHSA-3f8r-9483-pfxj

## [0.2.1] - 2021-05-01

### Added

- VerifyCanTerminate action

### Changed

- Updated to latest dependencies

### Security
- A privileged user (such as root) with local access to a server running WhiteBeam could kill the WhiteBeam logging service
  Fixed in 0.2.1: https://github.com/WhiteBeamSec/WhiteBeam/security/advisories/GHSA-h543-6328-8f64

## [0.2.0] - 2021-04-20

### Added

- Project changelog
- Linux LD_PRELOAD/LD_AUDIT library: Generic hook
- Linux LD_PRELOAD/LD_AUDIT library: Support for 40 hooks including Execution and Filesystem hooks
- Database-driven design
- Settings
- Commands to modify WhiteBeam settings, toggle hooks, and load SQL
- Modular action framework (compile time reflection), 12 actions
- Modular hash framework (compile time reflection), added hashing algorithms (ARGON2ID, BLAKE3, SHA-3)
- Hybrid hashing
- Recovery secret

### Changed

- Linux LD_PRELOAD/LD_AUDIT library: LD_AUDIT loader
- Replaced SodiumOxide with pure Rust audited cryptography library (RustCrypto)
- Improved whitelisting system
- Updated to latest dependencies

### Removed

- SHA-2 hash family

### Security
- A user with local access to a server running WhiteBeam could bypass whitelisting functionality
  Fixed in 0.2.0: https://github.com/WhiteBeamSec/WhiteBeam/security/advisories/GHSA-7wf6-3j4p-jm8x

## [0.1.3] - 2020-03-25

### Added

- Linux installer
- Linux LD_PRELOAD library: tests

### Changed

- Linux LD_PRELOAD library: refactored fexecve
- Project is now fully Rust
- Relicensed as CC-BY-NC
- Updated to latest dependencies

### Removed

- Dependency on GNU Make

### Fixed

- execl* corrected

## [0.1.2] - 2020-03-08

### Added

- Baselines
- Copyright, organization
- Hashing standardized to libsodium default (SHA3 removed)
- Linux LD_PRELOAD library: new hook templates, refactored hooks

### Removed

- Linux LD_PRELOAD library: original hook template

## [0.1.1] - 2020-02-01

### Added

- Exception handling
- Many new CLI arguments
- WhiteBeam service: updated to be asynchronous

### Changed

- Updated to latest dependencies

### Fixed

- Correct OS encoding of strings
- WhiteBeam service: execution log API restricted to localhost

## [0.1.0] - 2019-12-26

### Added

- libsodium cryptography
- Project code restructured into workspaces
- WhiteBeam service: encrypted API route, public key API route

### Changed

- Updated to latest dependencies

### Fixed

- Linux LD_PRELOAD library: warn on seccomp usage (fix scheduled)
- Optimized memory usage

## [0.0.9] - 2019-11-20

### Added

- CLI --status argument for monitoring service health
- Database initialization routines
- Dynamic whitelists
- Initial release binaries provided

### Changed

- Updated to latest dependencies

### Fixed

- execl* corrected

## [0.0.8] - 2019-10-15

### Added

- Cross platform support for uptime, locating data files
- Database functions, objects are now platform-independent
- Linux LD_PRELOAD library: hooks structured to be modular
- Prototype whitelist functionality working
- WhiteBeam library targets nightly Rust for variadic function support
- WhiteBeam service: startup script for Linux

## [0.0.7] - 2019-09-02

### Added

- Linux LD_PRELOAD library: file descriptor support
- Linux LD_PRELOAD library: hooks for exec family

### Fixed

- Error handling for hashing

## [0.0.6] - 2019-08-31

### Added

- Whitelisting and hashing of authorized executables

### Fixed

- Refactored library HTTP requests to reduce crashes

### Security
- If the LD_PRELOAD/LD_AUDIT environment variables were defined to a nonexecutable shared object library, execution of non-whitelisted library functions was possible.
  Fixed in 0.0.6: https://github.com/WhiteBeamSec/WhiteBeam/security/advisories/GHSA-mm3f-f5hg-p2hv

## [0.0.5] - 2019-08-26

### Added

- Created bug bounty
- Linux LD_PRELOAD library: Execution logging
- Reduced file size of release binaries
- WhiteBeam service: API endpoint to process executions (log/exec)

## [0.0.4] - 2019-08-10

### Added

- WhiteBeam service/CLI

## [0.0.3] - 2019-06-23

### Added

- Linux LD_PRELOAD library: execve support
- Linux LD_PRELOAD library: test case for execve

## [0.0.2] - 2019-05-20

### Added

- Linux LD_PRELOAD library: working function interposition
- Project code structured to be modular

## [0.0.1] - 2019-05-20

### Added

- Project license

[unreleased]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.9...v0.1.0
[0.0.9]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/WhiteBeamSec/WhiteBeam/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/WhiteBeamSec/WhiteBeam/releases/tag/v0.0.1
