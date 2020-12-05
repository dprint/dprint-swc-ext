// https://github.com/dsherret/tsconf-talk/blob/master/scripts/utils/nameToSnakeCase.ts
export function nameToSnakeCase(name: string) {
    let snakeCaseName = "";
    let canNextBeUnderscore = false;

    for (const char of name) {
        const isSeparatorChar = /[A-Z]/.test(char);
        if (isSeparatorChar && canNextBeUnderscore) {
            snakeCaseName += "_";
        }

        snakeCaseName += char.toLowerCase();
        canNextBeUnderscore = !isSeparatorChar && char !== "_";
    }

    return snakeCaseName;
}
