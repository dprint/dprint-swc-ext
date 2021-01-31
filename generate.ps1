# Ensure cargo-clone is installed (cargo install cargo-clone)
Remove-Item swc_ecma_ast -Recurse -ErrorAction Ignore
cargo clone --vers 0.37.3 swc_ecma_ast
cd swc_ecma_ast
cargo +nightly rustdoc -- --output-format json
cd ../
cp swc_ecma_ast/target/doc/swc_ecma_ast.json swc_ecma_ast.json
deno run -A generation/main.ts
