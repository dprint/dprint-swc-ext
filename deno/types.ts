import type { Token } from "./types.generated";

export abstract class Node {
    kind!: string;
    start: number;
    end: number;
    parent?: Node;

    protected constructor() {
        throw new Error("A node cannot be constructed.");
    }

    abstract getChildren(): Node[];
}

export interface Span {
    start: number;
    end: number;
}

export interface BigIntValue {
    // todo
}

export interface JsWord {
    // todo
}

export interface Comment {
    kind: CommentKind;
    start: number;
    end: number;
    text: string;
}

export enum CommentKind {
    Line,
    Block,
}

export interface TokenAndSpan {
    start: number;
    end: number;
    hadLineBreak: boolean;
    token: Token;
}
