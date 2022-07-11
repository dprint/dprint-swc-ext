#!/usr/bin/env -S deno run -A
import $ from "https://raw.githubusercontent.com/dsherret/ax/b6dbab0a4c3028538a38fa948df3025605098f19/mod.ts";

if (!Deno.args.some(a => a === "--quick")) {
  const swcVersions = await getSwcVersions();
  $.echo(`Setting up crates. Note: Provide --quick to just code generate.`);
  $.echo(`Setting up swc_ecma_ast ${swcVersions.swcEcmaAst}...`);
  await $.fs.emptyDir("swc_ecma_ast");
  await $`cargo clone swc_ecma_ast@${swcVersions.swcEcmaAst}`;
  await $`cd swc_ecma_ast ; cargo +nightly rustdoc -- --output-format json -Z unstable-options`;
  await $`cp swc_ecma_ast/target/doc/swc_ecma_ast.json swc_ecma_ast.json`;

  $.echo(`Setting up swc_ecma_parser ${swcVersions.swcEcmaParser}...`);
  await $.fs.emptyDir("swc_ecma_parser");
  await $`cargo clone swc_ecma_parser@${swcVersions.swcEcmaParser}`;
  // generate these files to make cargo happy
  await $.fs.ensureFile("swc_ecma_parser/benches/compare/main.rs");
  await $.fs.ensureFile("swc_ecma_parser/benches/parser/main.rs");
  await $.fs.ensureFile("swc_ecma_parser/benches/lexer/main.rs");
  await $`cd swc_ecma_parser ; cargo +nightly rustdoc -- --output-format json -Z unstable-options`;
  await $`cp swc_ecma_parser/target/doc/swc_ecma_parser.json swc_ecma_parser.json`;
}

$.echo("Generating code...");
await $`deno run -A generation/main.ts`.stdout("inherit");

async function getSwcVersions() {
  const output = await $`cargo metadata --format-version 1`;
  return {
    swcEcmaAst: getPackageOrThrow("swc_ecma_ast").version,
    swcEcmaParser: getPackageOrThrow("swc_ecma_parser").version,
  };

  function getPackageOrThrow(name: string) {
    const pkg = output.stdoutJson.packages.find((pkg: any) => pkg.name === name);
    if (pkg == null) {
      throw new Error(`Could not find package: ${name}`);
    }
    return pkg;
  }
}
