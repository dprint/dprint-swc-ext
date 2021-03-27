import type { AnalysisResult } from "./analysis_types.ts";
import { analyzeAstCrate } from "./analyze_ast_crate.ts";
import { analyzeParserCrate } from "./analyze_parser_crate.ts";

export function analyze(): AnalysisResult {
    const astCrate = analyzeAstCrate();
    const parserCrate = analyzeParserCrate();

    return {
        tokenEnums: parserCrate.tokenEnums,
        astEnums: astCrate.astEnums,
        astStructs: astCrate.astStructs,
        plainEnums: astCrate.plainEnums,
    };
}
