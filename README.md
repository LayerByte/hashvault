# Hashvault

Fast file hashing utility.

School Purpose Only.

## Overview

Hashvault is a defensive cybersecurity and system-administration project written in Stable Rust. It focuses on read-only analysis, validation, clear output, and safe handling of user-provided files, URLs, hosts, or text.

## Features

- Defensive, educational workflow
- Read-only file and network analysis where applicable
- Input validation and friendly error messages
- No credential collection, payload delivery, exploitation, or destructive actions
- Small, understandable codebase suitable for students

## Requirements

- Rust stable toolchain
- Cargo

## Installation

Clone or copy this project folder, then follow the standard build or run command for Stable Rust.

## Usage

`cargo run -- sample.bin`
`cargo run -- 192.168.1.0/24`

## Example

```text
cargo run -- sample.bin --strings
```

## How It Works

The tool validates input, performs a focused defensive analysis task, and prints a clear report. File operations are read-only and network operations use normal platform TLS and timeout behavior.

## Security Considerations

- Do not run tools against systems without permission.
- Do not paste secrets into command-line arguments.
- Review output before sharing it because file paths and hostnames may be sensitive.
- Network tools do not disable certificate validation.

## Limitations

- Designed for education and local administration workflows.
- Not a replacement for enterprise security monitoring.
- Some platform-specific data may vary by operating system.

## Disclaimer

This project is for defensive learning, school assignments, and authorized administration. It does not include malware, credential theft, brute-force attacks, exploitation, payload delivery, persistence, or bypass functionality.

## License

Released under the MIT License.
