export class Node {
    kind!: string;
    span: Span;
    parent?: Node;

    protected constructor() {
        throw new Error("A node cannot be constructed.");
    }
}

export interface Span {
    lo: number;
    hi: number;
    ctx: number;
}

export interface BigIntValue {
    // todo
}

export interface JsWord {
    // todo
}
