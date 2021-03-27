export function nameToSnakeCase(name: string) {
    // https://github.com/dsherret/tsconf-talk/blob/master/scripts/utils/nameToSnakeCase.ts
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

export function snakeCaseToCamel(name: string) {
    const parts = name.split("_");
    return parts[0] + parts.slice(1).map(name => name[0].toUpperCase() + name.slice(1)).join("");
}
