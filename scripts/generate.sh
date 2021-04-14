#!/bin/bash

set -eu

# Analyze
if [[ $* != *--quick* ]]
then
    # Ensure cargo-clone is installed (cargo install cargo-clone)
    VERSIONS_RESULT=$(deno run --allow-run scripts/get-swc-ecma-ast-version.ts)
    SWC_AST_VERSION=$(awk -F_ '{print $1}' <<< $VERSIONS_RESULT)
    SWC_PARSER_VERSION=$(awk -F_ '{print $2}' <<< $VERSIONS_RESULT)

    echo "Setting up swc_ecma_ast $SWC_AST_VERSION... provide --quick to just code generate"
    rm -rf swc_ecma_ast
    cargo clone --vers $SWC_AST_VERSION swc_ecma_ast
    (cd swc_ecma_ast && cargo +nightly rustdoc -- --output-format json -Z unstable-options)
    cp swc_ecma_ast/target/doc/swc_ecma_ast.json swc_ecma_ast.json

    echo "Setting up swc_ecma_parser $SWC_PARSER_VERSION..."
    rm -rf swc_ecma_parser
    cargo clone --vers $SWC_PARSER_VERSION swc_ecma_parser
    (cd swc_ecma_parser && cargo +nightly rustdoc -- --output-format json -Z unstable-options)
    cp swc_ecma_parser/target/doc/swc_ecma_parser.json swc_ecma_parser.json

    echo "Generating code..."
fi

# Generate
deno run -A generation/main.ts
