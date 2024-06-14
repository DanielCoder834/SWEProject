// @author Alvin Wong 
type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "CHARACTERS" | "NUMBER";

export interface Token {
    type: TokenType;
    value: string;
}

export class Tokenize {
    private input: string;
    private currentIndex: number;
    private static readonly numberRegex = /[0-9.]/;
    private static readonly operatorRegex = /[+\-*/<>=&|:(),]/;
    private static readonly cellReferenceRegex = /[$A-Za-z]/;
    private static readonly stringRegex = /^"([^"]*)"/;

    constructor(input: string) {
        this.input = input;
        this.currentIndex = 0;
    }

    // @author Adarsh Jayaram
    tokenize(): Token[] {
        const tokens: Token[] = [];
        while (this.currentIndex < this.input.length) {
            const char = this.input[this.currentIndex];
            switch (true) {
                case char.match(Tokenize.numberRegex) != null:
                    tokens.push(this.tokenizeNum());
                    break;
                case this.input.startsWith("<>", this.currentIndex):
                    tokens.push(this.tokenizeOperator('<>'));
                    this.currentIndex += 2; // Move past the two-character operator
                    continue;
                case char.match(Tokenize.operatorRegex) != null:
                    tokens.push(this.tokenizeOperator());
                    break;
                case char.match(Tokenize.cellReferenceRegex) != null:
                    tokens.push(this.isRef() ? this.tokenizeRef() : this.tokenizeFunc());
                    break;
                case char === '"':
                    tokens.push(this.tokenizeChars());
                    break;
                case /\s/.test(char):
                    tokens.push({ type: "CHARACTERS", value: " " }); // Optionally track whitespace
                    this.currentIndex++; // Skip whitespace
                    break;
                default:
                    throw new Error(`Unknown character: ${char}`);
            }
        }
    
        tokens.push({ type: "FORMULA", value: "" }); // Append EOF token represented as FORMULA
        return tokens;
    }

    private tokenizeNum(): Token {
        let number = '';
        while (this.currentIndex < this.input.length && this.input[this.currentIndex].match(/[0-9.]/)) {
            number += this.input[this.currentIndex++];
        }
        return { type: "NUMBER", value: number };
    }

    private tokenizeOperator(operator?: string): Token {
        if (operator) {
            this.currentIndex += operator.length;  // Adjust index for multi-character operators like '<>'
            return { type: "OPERATOR", value: operator };
        }
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
        const endQuoteIndex = this.input.indexOf('"', this.currentIndex + 1);
        if (endQuoteIndex === -1) {
            throw new Error('Unterminated string');
        }
        const str = this.input.substring(this.currentIndex + 1, endQuoteIndex);
        this.currentIndex = endQuoteIndex + 1; // Move past the end quote
        return { type: "CHARACTERS", value: str };
    }

    private isRef(): boolean {
        const pattern = /^\$?[A-Za-z]+\$?[0-9]+/;
        return pattern.test(this.input.substring(this.currentIndex));
    }
}
