# Contributing

## Windows

If you're on Windows, use WSL.

## Setup

1. Install [cargo-clone](https://crates.io/crates/cargo-clone): `cargo install cargo-clone`
2. Either do the following:
   - Install [bvm](https://github.com/bvm/bvm) and run `bvm install` in the root directory.
   - ALTERNATIVELY instead of installing bvm, install the following binaries on their own:
     - [Deno](https://deno.land) - For running the code generation script.
     - [dprint](https://dprint.dev) - For code formatting.

## Code Generating

1. Run `./scripts/setup.sh` once in the root of the repo and whenever swc's version changes.
2. After that, run `./scripts/generate.sh` to code generate (initially and whenever you make changes to the code generation code).
