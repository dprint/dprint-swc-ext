# Contributing

## Setup

1. Install [cargo-clone](https://crates.io/crates/cargo-clone): `cargo install cargo-clone`
1. Install [Deno](https://deno.land) - For running the code generation script.
1. Install [dprint](https://dprint.dev) - For code formatting.

## Code Generating

1. Run `./scripts/generate.ts` once in the root of the repo and whenever swc's version changes to analyze swc then code generate.
1. After that, you only need to run `./scripts/generate.ts --quick` to code generate.
