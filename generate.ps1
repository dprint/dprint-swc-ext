cd swc/ecmascript/ast
cargo +nightly rustdoc -- --output-format json
cd ../../../
cp swc/target/doc/swc_ecma_ast.json swc_ecma_ast.json
deno run -A generation/main.ts
