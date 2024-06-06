/*
Types are defined as follows:
    1. REF - A string consisting of a dollar sign ($), column identifier (case-insensitive sequence
        of alphabetic letters), and row identifier (nonzero positive integer).
    2. FORMULA - A string that starts with an equal sign (=) and followed by an expression.
    3. OPERATOR - A mathematical operator. Supports +, -, *, /, <, >, =, <>, &, |, and :.
    4. FUNCTION - A spreadsheet function. Supports IF, SUM, MIN, MAX, AVG, CONCAT, and DEBUG.
    5. WHITESPACE - Any whitespace character used to separate terms.
*/
type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "WHITESPACE";
 
interface Token {
    type: TokenType;
    val: string;
}
 
// Class that handles tokenization of input.
export default class Parser {
    private input: string;
    private tokens: Token[] = [];
    // Regex patterns created with help from https://regex-generator.olafneumann.org/?sampleText=&flags=i and ChatGPT
    private patterns: { type: TokenType; pattern: RegExp }[] = [
        { type: "REF", pattern: /\$[A-Za-z]+[1-9]\d*/ },
        { type: "FORMULA", pattern: /=\s*(?:[+\-*/()]|\b\d+\b|\b[A-Za-z_][A-Za-z0-9_]*\b)+\s*/ },
        { type: 'OPERATOR', pattern: /[+\-*\/()<>=:&|]|<>|<=|>=|:=/ },
        { type: 'FUNCTION', pattern: /\b(IF|SUM|MIN|MAX|AVG|CONCAT|DEBUG)\s*\((.*?)\)/i },
        { type: 'WHITESPACE', pattern: /\s+/ },
    ];
 
    constructor(input: string) {
        this.input = input;
        this.tokenize();
    }
 
    // Tokenizes the entire input string, checks it against known patterns, and stores them in the tokens array.
    private tokenize() {
        let pos = 0;
        while (pos < this.input.length) {
            let match: RegExpExecArray | null = null;
            let matched = false;
 
            for (const { type, pattern } of this.patterns) {
                match = pattern.exec(this.input);
 
                if (match && match.index === pos) {
                    if (type !== 'WHITESPACE') {
                        this.tokens.push({ type, val: match[0].trim()})
                    }
                    pos += match[0].length;
                    matched = true;
                    break;
                }
            }
 
            if (!matched) {
                break;
            }
        }
    }
 
    // Getter to retrieve all tokens.
    public getTokens(): Token[] {
        return this.tokens;
    }
}