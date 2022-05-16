import { analyze } from "./analyze/analyze.ts";
import { generate } from "./generate/generate.ts";

const analysisResult = analyze();

Deno.writeTextFileSync("./crates/swc-ecma-ast-view/src/generated.rs", generate(analysisResult));
