// Base interface for all nodes, specifying that each node must have a type identifier.
// @author Adarsh Jayaram
/* 
Writing a parser is, depending on the language, a moderately complex task. In essence,
it must transform a piece of code (which we inspect by looking at the characters) into an
“abstract syntax tree” (AST). The AST is a structured in-memory representation of the program,
and it's “abstract” in the sense that it does not care exactly what characters is the source
code made of, but it faithfully represents the semantics of it. I wrote a separate page
to describe our AST.
Reference for AST logic:
https://lisperator.net/pltut/parser/
*/
export interface ASTNode {
    type: string;
}

// Represents a numeric value in the AST.
export class NumberNode implements ASTNode {
    readonly type = "Number";

    constructor(public value: number) {}
}

// Represents binary operations such as addition, subtraction, multiplication, or division.
export class BasicOperationNode implements ASTNode {
    readonly type = "BasicOperation";

    constructor(
        public operator: string,
        public left: ASTNode,
        public right: ASTNode
    ) {}
}

// Represents function calls within the AST.
export class FunctionNode implements ASTNode {
    type = "Function";
    functionName: string;
    arguments: ASTNode[];

    constructor(functionName: string, args: ASTNode[]) {
        this.functionName = functionName;
        this.arguments = args;
    }
}

// Represents references to other cells within the AST, potentially for use in formulas.
export class ReferenceNode implements ASTNode {
    readonly type = "Ref";

    constructor(public reference: string) {}
}

// Represents string literals within the AST.
export class StringNode implements ASTNode {
    readonly type = "String";

    constructor(public value: string) {}
}
