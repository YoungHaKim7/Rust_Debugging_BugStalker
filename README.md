# Rust_Debugging_BugStalker

# BugStalker
- Modern debugger for Linux x86-64. Written in Rust for Rust programs.
  - https://github.com/godzie44/BugStalker


# Install

- First check if the necessary dependencies (pkg-config and libunwind-dev) are installed:

- For example, ubuntu/debian:

```bash
apt install pkg-config libunwind-dev
```

- For example, fedora:

```
dnf install pkg-config libunwind-devel
```

- Now install debugger:
```bash
cargo install bugstalker
```

# 사용법
- https://github.com/godzie44/BugStalker/tree/master/doc

<hr>

# tokio console

- a debugger for async rust!

  - https://github.com/tokio-rs/console