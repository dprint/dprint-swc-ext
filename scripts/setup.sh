#!/bin/sh

# Ensure cargo-clone is installed (cargo install cargo-clone)
rm -rf swc_ecma_ast
SWC_VERSION=$(deno run --allow-run scripts/get-swc-ecma-ast-version.ts)

cargo clone --vers $SWC_VERSION swc_ecma_ast
(cd swc_ecma_ast && cargo +nightly rustdoc -- --output-format json)
cp swc_ecma_ast/target/doc/swc_ecma_ast.json swc_ecma_ast.json
