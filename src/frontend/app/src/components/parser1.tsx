import { ASTNode, NumberNode, BasicOperationNode, FunctionNode, ReferenceNode, StringNode } from "./astnodes";

type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "CHARACTERS" | "NUMBER";

export interface Token {
    type: TokenType;
    value: string;
}

export class Parser {
    private tokenStream: Token[];
    private currentPos: number;

    // Constructor: Initializes the parser with a stream of tokens
    constructor(tokens: Token[]) {
        this.tokenStream = tokens;
        this.currentPos = 0;
    }

    // Main parse function that processes tokens into an AST
    parse(): ASTNode {
        let node = this.factor(); // Start with a factor, could be a number, a parenthesis expression, etc.

        // Process any multiplication/division operations next as they have higher precedence
        while (this.check("OPERATOR", "*", "/")) {
            const operator = this.getNextToken().value;
            const right = this.factor();
            node = new BasicOperationNode(operator, node, right);
        }

        // Process addition and subtraction after higher precedence operations
        while (this.check("OPERATOR", "+", "-", "<", ">", "=", "<>", "&", "|", ":")) {
            const operator = this.getNextToken().value;
            let nextFactor = this.factor();
            while (this.check("OPERATOR", "*", "/")) { // Inner loop for nested multiplication/division
                const subOperator = this.getNextToken().value;
                const subRight = this.factor();
                nextFactor = new BasicOperationNode(subOperator, nextFactor, subRight);
            }
            node = new BasicOperationNode(operator, node, nextFactor);
        }

        return node;
    }

    // Processes individual factors in expressions
    factor(): ASTNode {
        if (this.check("NUMBER")) { // If it's a number, create a NumberNode
            return new NumberNode(parseFloat(this.getNextToken().value));
        }

        if (this.check("OPERATOR", "(")) { // Handle expressions inside parentheses
            this.getNextToken(); // Consume '('
            const expression = this.parse(); // Recursively parse the expression
            if (!this.check("OPERATOR", ")")) {
                throw new Error("Expected ')' to close expression");
            }
            this.getNextToken(); // Consume ')'
            return expression;
        }

        if (this.check("FUNCTION")) { // Handle function calls
            const functionName = this.getNextToken().value;
            if (!this.check("OPERATOR", "(")) {
                throw new Error("Expected '(' after function name");
            }
            this.getNextToken(); // Consume '('
            const args = this.parseFunctionArguments(); // Parse function arguments
            if (!this.check("OPERATOR", ")")) {
                throw new Error("Expected ')' after function arguments");
            }
            this.getNextToken(); // Consume ')'
            return new FunctionNode(functionName, args);
        }

        if (this.check("REF")) { // Handle references (like variables or cell references)
            return new ReferenceNode(this.getNextToken().value);
        }

        if (this.check("CHARACTERS")) { // Handle character strings
            return new StringNode(this.getNextToken().value);
        }

        // Throw an error if an unexpected token is encountered
        throw new Error(`Unexpected token: ${this.tokenStream[this.currentPos]?.value || 'EOF'}`);
    }

    // Helper function to parse arguments of a function
    parseFunctionArguments(): ASTNode[] {
        const args: ASTNode[] = [];
        while (!this.check("OPERATOR", ")")) {
            args.push(this.parse());
            if (this.check("OPERATOR", ",")) {
                this.getNextToken(); // Consume ','
            } else {
                break; // Stop if no comma, expecting ')'
            }
        }
        return args;
    }    

    // Checks if the current token matches the expected type and optional values
    check(type: TokenType, ...expectedValues: string[]): boolean {
        const token = this.tokenStream[this.currentPos];
        if (!token || token.type !== type) {
            return false;
        }
        return expectedValues.length === 0 || expectedValues.includes(token.value);
    }

    // Retrieves the next token in the stream, advancing the position
    getNextToken(): Token {
        if (this.currentPos >= this.tokenStream.length) {
            throw new Error("Attempted to access a token beyond the end of the stream.");
        }
        return this.tokenStream[this.currentPos++];
    }
}
