// @author Alvin Wong
// Token types supported by the tokenizer.
type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "CHARACTERS" | "NUMBER";

// Token structure representing the type and value of each piece of the input.
export interface Token {
    type: TokenType;
    value: string;
}

export class Tokenize {
    private input: string;
    private currentIndex: number = 0;
    private static readonly numberRegex = /[0-9.]/;
    private static readonly operatorRegex = /[+\-*/<>=&|:(),]/;
    private static readonly cellReferenceRegex = /[$A-Za-z]/;
    private static readonly stringRegex = /^"([^"]*)"/;

    constructor(input: string) {
        this.input = input;
    }
    // @author Adarsh Jayaram
    tokenize(): Token[] {
        const tokens: Token[] = [];
        while (this.currentIndex < this.input.length) {
            const char = this.input[this.currentIndex];
            if (/\s/.test(char)) {
                this.currentIndex++; // Ignore whitespaces
                continue;
            }

            if (char.match(Tokenize.numberRegex)) {
                tokens.push(this.tokenizeNum());
            } else if (this.input.startsWith("<>", this.currentIndex)) {
                tokens.push({ type: "OPERATOR", value: "<>" });
                this.currentIndex += 2; // Advance past the "<>" operator
            } else if (char.match(Tokenize.operatorRegex)) {
                tokens.push(this.tokenizeOperator());
            } else if (char.match(Tokenize.cellReferenceRegex)) {
                if (this.lookAheadForRef()) {
                    tokens.push(this.tokenizeRef());
                } else {
                    tokens.push(this.tokenizeFunc());
                }
            } else if (char === '"') {
                tokens.push(this.tokenizeChars());
            } else {
                throw new Error(`Unknown character: ${char}`);
            }
        }

        tokens.push({ type: "FORMULA", value: "" }); // Append EOF token as FORMULA
        return tokens;
    }

    private tokenizeNum(): Token {
        let number = '';
        while (this.currentIndex < this.input.length && this.input[this.currentIndex].match(/[0-9.]/)) {
            number += this.input[this.currentIndex++];
        }
        return { type: "NUMBER", value: number };
    }

    private tokenizeOperator(): Token {
        const value = this.input[this.currentIndex++];
        return { type: "OPERATOR", value: value };
    }

    private tokenizeRef(): Token {
        let reference = '';
        while (this.currentIndex < this.input.length && /[A-Za-z0-9\$]/.test(this.input[this.currentIndex])) {
            reference += this.input[this.currentIndex++];
        }
        return { type: "REF", value: reference };
    }

    private tokenizeFunc(): Token {
        let func = '';
        while (this.currentIndex < this.input.length && /[A-Za-z]/.test(this.input[this.currentIndex])) {
            func += this.input[this.currentIndex++];
        }
        return { type: "FUNCTION", value: func };
    }

    private tokenizeChars(): Token {
        this.currentIndex++; // Move past the opening quote
        let str = '';
        while (this.currentIndex < this.input.length && this.input[this.currentIndex] !== '"') {
            str += this.input[this.currentIndex++];
        }
        this.currentIndex++; // Move past the closing quote
        return { type: "CHARACTERS", value: str };
    }

    private lookAheadForRef(): boolean {
        const pattern = /^\$?[A-Za-z]+\$?[0-9]+/;
        const remainingInput = this.input.substring(this.currentIndex);
        return pattern.test(remainingInput);
    }
}
