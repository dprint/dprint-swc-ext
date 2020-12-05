#rm -rf swc_ast
#svn export https://github.com/swc-project/swc/trunk/ecmascript/ast swc_ast
cd swc/ecmascript/ast
cargo +nightly rustdoc -- --output-format json
cd ../../../
cp swc/target/doc/swc_ecma_ast.json swc_ecma_ast.json
