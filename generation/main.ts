import { analyze } from "./analyze/analyze.ts";
import { generate } from "./generate/generate.ts";
import { generateSerialize } from "./generate/generate_serialize.ts";
import { generateTypeScriptSetup, generateTypeScriptTypes } from "./generate/generate_ts.ts";

const analysisResult = analyze();

Deno.writeTextFileSync("./rs-lib/src/generated.rs", generate(analysisResult));
Deno.writeTextFileSync("./rs-lib/src/serialize/serialize_generated.rs", generateSerialize(analysisResult));
Deno.writeTextFileSync("./deno/types.generated.ts", generateTypeScriptTypes(analysisResult));
Deno.writeTextFileSync("./deno/setup.generated.ts", generateTypeScriptSetup(analysisResult));
