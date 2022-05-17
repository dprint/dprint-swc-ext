import { analyze } from "./analyze/analyze.ts";
import { generate } from "./generate/generate.ts";

const analysisResult = analyze();

Deno.writeTextFileSync("./rs-lib/src/view/generated.rs", generate(analysisResult));
