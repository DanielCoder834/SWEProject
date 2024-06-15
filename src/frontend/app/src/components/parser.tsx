/*
Types are defined as follows:
    1. REF - A string consisting of a dollar sign ($), column identifier (case-insensitive sequence
        of alphabetic letters), and row identifier (nonzero positive integer).
    2. FORMULA - A string that starts with an equal sign (=) and followed by an expression.
    3. OPERATOR - A mathematical operator. Supports +, -, *, /, <, >, =, <>, &, |, and :.
    4. FUNCTION - A spreadsheet function. Supports IF, SUM, MIN, MAX, AVG, CONCAT, and DEBUG.
    5. WHITESPACE - Any whitespace character used to separate terms.
*/

type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "WHITESPACE" | "NUMBER";

export interface Token {
    type: TokenType;
    val: string;
}

// Class that handles tokenization of input.

export default class Parser {
    private input: string;
    private tokens: Token[] = [];

    // Adjusted regex patterns to ensure proper capturing
    private patterns: { type: TokenType; pattern: RegExp }[] = [
        { type: "REF", pattern: /\$[A-Za-z]+[1-9][0-9]*/ },
        { type: "FORMULA", pattern: /=/ }, // Simple catch for formula start, handling in logic
        { type: "OPERATOR", pattern: /[+\-*\/<>=:&|]|<>/ },
        { type: "NUMBER", pattern: /\b\d+(\.\d+)?\b/ }, // Matches integers and decimals
        { type: "WHITESPACE", pattern: /\s+/ },
    ];

    constructor(input: string) {
        this.input = input;
        this.tokenize();
    }

    private tokenize() {
        let pos = 0;
        while (pos < this.input.length) {
            let matched = false;
    
            for (const { type, pattern } of this.patterns) {
                const match = pattern.exec(this.input.substring(pos));
    
                if (match) {
                    if (type !== 'WHITESPACE') {
                        this.tokens.push({ type, val: match[0] });
                        console.log(`Token added: Type=${type}, Value=${match[0]}`);
                    }
                    pos += match[0].length;
                    matched = true;
                    break;
                }
            }
    
            if (!matched) {
                console.error("Unrecognized character or sequence at position:", pos);
                break;
            }
        }
    }

    public getTokens(): Token[] {
        console.log("Tokens generated:", this.tokens);
        return this.tokens;
    }
}