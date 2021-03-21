import { analyze } from "./analyze/analyze.ts";
import { generate } from "./generate/generate.ts";
import { generateSerialize } from "./generate/generate_serialize.ts";

const analysisResult = analyze();
const generatedCode = generate(analysisResult);
const generatedSerializeCode = generateSerialize(analysisResult);

Deno.writeTextFileSync("./rs-lib/src/generated.rs", generatedCode);
Deno.writeTextFileSync("./rs-lib/src/generated_serialize.rs", generatedSerializeCode);
