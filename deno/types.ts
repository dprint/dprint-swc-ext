import type { Node, NodeKind, Token } from "./types.generated.ts";

export abstract class BaseNode {
    kind!: NodeKind;
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
