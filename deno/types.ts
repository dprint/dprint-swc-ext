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

// todo: I'm not sure what the type here actually is and I'm lazy. Please submit a PR if you know.
export type BigIntValue = [number, [number]];

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
