// Very primitive... would be nice to port code-block-writer to Deno even though it is JS/TS specific
export class Writer {
    private text: string[] = [];
    private isAtStartOfLine = false;
    private indentLevel = 0;

    write(text: string) {
        if (this.isAtStartOfLine) {
            this.writeIndent();
            this.isAtStartOfLine = false;
        }

        this.append(text);

        return this;
    }

    newLine() {
        this.text.push("\n");
        this.isAtStartOfLine = true;
        return this;
    }

    indent(action: () => void) {
        this.indentLevel++;
        action();
        this.indentLevel--;
        return this;
    }

    private writeIndent() {
        this.append("  ".repeat(this.indentLevel));
    }

    private append(text: string) {
        if (text.length > 0) {
            this.text.push(text);
        }
    }

    toString() {
        return this.text.join("");
    }
}
