# Turbolift Example Repo

[![ci](https://github.com/DominicBurkart/turbolift_example/actions/workflows/ci.yaml/badge.svg)](https://github.com/DominicBurkart/turbolift_example/actions/workflows/ci.yaml)

This repo exists to test that [Turbolift](https://dominic.computer/turbolift) works when installed via cargo instead of via source.
If you'd like, you can use it as a template for getting started with a Turbolift project! 

Remember that, as of writing, some of the span manipulation done by turbolift requires an unstable feature. Include 
the relevant compiler flag (`--cfg procmacro2_semver_exempt`) and an optional macro backtrace feature 
(`-Z macro-backtrace`) while running or building your project: 
```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt -Z macro-backtrace' cargo run +nightly --features distributed
# or
RUSTFLAGS='--cfg procmacro2_semver_exempt -Z macro-backtrace' cargo build +nightly --features distributed
```
