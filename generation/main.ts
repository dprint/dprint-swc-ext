import { analyze } from "./analyze/analyze.ts";
import { generate } from "./generate/generate.ts";

const analysisResult = analyze();
const generatedCode = generate(analysisResult);

Deno.writeTextFileSync("./rs-lib/src/lib.rs", generatedCode);
