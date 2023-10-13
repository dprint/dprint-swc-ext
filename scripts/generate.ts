#!/usr/bin/env -S deno run -A
import $ from "https://deno.land/x/dax@0.35.0/mod.ts";

$.setPrintCommand(true);

const root = $.path(import.meta).join("../../").resolve();

if (!Deno.args.some(a => a === "--quick")) {
  const swcVersions = await getSwcVersions();
  $.logStep("Setting up crates. Note: Provide --quick to just code generate.");
  $.logStep(`Setting up swc_ecma_ast ${swcVersions.swcEcmaAst}...`);
  const astDir = root.join("swc_ecma_ast");
  astDir.emptyDirSync();
  await $`cargo clone --version`;
  await $`cargo clone swc_ecma_ast@${swcVersions.swcEcmaAst}`;
  // force using an old version of the regex crate that works in Rust 1.65
  const cargoFile = astDir.join("Cargo.toml");
  cargoFile.writeTextSync(
    cargoFile.readTextSync()
      + "[dependencies.regex]\nversion = \"=1.5.5\"\n",
  );
  await $`cd swc_ecma_ast ; cargo rustdoc -- --output-format json -Z unstable-options`;
  astDir.join("target/doc/swc_ecma_ast.json")
    .copyFileSync(root.join("swc_ecma_ast.json"));

  $.logStep(`Setting up swc_ecma_parser ${swcVersions.swcEcmaParser}...`);
  const parserDir = root.join("swc_ecma_parser");
  parserDir.emptyDirSync();
  await $`cargo clone swc_ecma_parser@${swcVersions.swcEcmaParser}`;
  // generate these files to make cargo happy
  parserDir.join("benches/compare/main.rs").ensureFileSync();
  parserDir.join("benches/parser/main.rs").ensureFileSync();
  parserDir.join("benches/lexer/main.rs").ensureFileSync();
  await $`cd swc_ecma_parser ; cargo rustdoc -- --output-format json -Z unstable-options`;
  parserDir.join("target/doc/swc_ecma_parser.json")
    .copyFileSync(root.join("swc_ecma_parser.json"));
}

$.logStep("Generating", "code...");
await $`deno run -A generation/main.ts`;

async function getSwcVersions() {
  const output = await $`cargo metadata --format-version 1`.json();
  return {
    swcEcmaAst: getPackageOrThrow("swc_ecma_ast").version,
    swcEcmaParser: getPackageOrThrow("swc_ecma_parser").version,
  };

  function getPackageOrThrow(name: string) {
    const pkg = output.packages.find((pkg: any) => pkg.name === name);
    if (pkg == null) {
      throw new Error(`Could not find package: ${name}`);
    }
    return pkg;
  }
}
