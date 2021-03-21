import { Writer } from "../deps.ts";

export function createWriter() {
    return new Writer({
        indentNumberOfSpaces: 2,
    });
}
