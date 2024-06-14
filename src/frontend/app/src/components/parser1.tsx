import { ASTNode, NumberNode, BasicOperationNode, FunctionNode, ReferenceNode, StringNode } from "./astnodes";

// @author Alivin Wong
type TokenType = "REF" | "FORMULA" | "OPERATOR" | "FUNCTION" | "CHARACTERS" | "NUMBER";

export interface Token {
    type: TokenType;
    value: string;
}

// @author Adarsh Jayaram
export class Parser {

    private tokenStream: Token[];
    private currentPos: number;

    constructor(tokens: Token[]) {
        this.tokenStream = tokens;
        this.currentPos = 0;
    }

    parse(): ASTNode {
        let node = this.factor();

        while (this.check("OPERATOR", "*", "/")) {
            const operator = this.getNextToken().value;
            const right = this.factor();
            node = new BasicOperationNode(operator, node, right);
        }

        while (this.check("OPERATOR", "+", "-", "<", ">", "=", "<>", "&", "|", ":")) {
            const operator = this.getNextToken().value;
            let nextFactor = this.factor();
            while (this.check("OPERATOR", "*", "/")) {
                const subOperator = this.getNextToken().value;
                const subRight = this.factor();
                nextFactor = new BasicOperationNode(subOperator, nextFactor, subRight);
            }
            node = new BasicOperationNode(operator, node, nextFactor);
        }

        return node;
    }

    factor(): ASTNode {
        if (this.check("NUMBER")) {
            return new NumberNode(parseFloat(this.getNextToken().value));
        }

        if (this.check("OPERATOR", "(")) {
            this.getNextToken(); // Consume '('
            const expression = this.parse(); // Recursively parse the enclosed expression.
            if (!this.check("OPERATOR", ")")) {
                throw new Error("Expected ')' to close expression");
            }
            this.getNextToken(); // Consume ')'
            return expression;
        }

        if (this.check("FUNCTION")) {
            const functionName = this.getNextToken().value;
            if (!this.check("OPERATOR", "(")) {
                throw new Error("Expected '(' after function name");
            }
            this.getNextToken(); // Consume '('
            const args = this.parseFunctionArguments(); // Parse function arguments.
            if (!this.check("OPERATOR", ")")) {
                throw new Error("Expected ')' after function arguments");
            }
            this.getNextToken(); // Consume ')'
            return new FunctionNode(functionName, args);
        }

        if (this.check("REF")) {
            return new ReferenceNode(this.getNextToken().value);
        }

        if (this.check("CHARACTERS")) {
            return new StringNode(this.getNextToken().value);
        }

        throw new Error(`Unexpected token: ${this.tokenStream[this.currentPos]?.value || 'EOF'}`);
    }

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

    check(type: TokenType, ...expectedValues: string[]): boolean {
        const token = this.tokenStream[this.currentPos];
        // Return false immediately if token is undefined or type doesn't match
        if (!token || token.type !== type) {
            return false;
        }
        // If expectedValues is empty, type match is sufficient, otherwise value must also match
        return expectedValues.length === 0 || expectedValues.includes(token.value);
    }

    getNextToken(): Token {
        if (this.currentPos >= this.tokenStream.length) {
            throw new Error("Attempted to access a token beyond the end of the stream.");
        }
        return this.tokenStream[this.currentPos++];
    }
}
