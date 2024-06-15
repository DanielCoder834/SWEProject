import { Parser, Token } from '../components/parser1';
import { ASTNode, NumberNode, BasicOperationNode, FunctionNode, ReferenceNode, StringNode } from "../components/astnodes";

// @author Alvin Wong
// Tests the methods in parser1.tsx
// Each test is labeled with the specific functionality it checks for.
describe('Parser Tests', () => {

    // Positive Tests
    test('Parses number nodes correctly', () => {
        const tokens: Token[] = [
            { type: 'NUMBER', value: '123' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new NumberNode(123));
    });

    test('Parses basic operation nodes correctly', () => {
        const tokens: Token[] = [
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: '*' },
            { type: 'NUMBER', value: '3' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new BasicOperationNode('*', new NumberNode(2), new NumberNode(3)));
    });

    test('Parses nested operations correctly', () => {
        const tokens: Token[] = [
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: '*' },
            { type: 'NUMBER', value: '3' },
            { type: 'OPERATOR', value: '+' },
            { type: 'NUMBER', value: '4' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new BasicOperationNode(
            '+',
            new BasicOperationNode('*', new NumberNode(2), new NumberNode(3)),
            new NumberNode(4)
        ));
    });

    test('Parses function nodes correctly', () => {
        const tokens: Token[] = [
            { type: 'FUNCTION', value: 'SUM' },
            { type: 'OPERATOR', value: '(' },
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: ')' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new FunctionNode('SUM', [new NumberNode(1), new NumberNode(2)]));
    });

    test('Parses reference nodes correctly', () => {
        const tokens: Token[] = [
            { type: 'REF', value: '$A1' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new ReferenceNode('$A1'));
    });

    test('Parses string nodes correctly', () => {
        const tokens: Token[] = [
            { type: 'CHARACTERS', value: 'Hello' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new StringNode('Hello'));
    });

    test('Parses mixed expressions correctly', () => {
        const tokens: Token[] = [
            { type: 'FUNCTION', value: 'SUM' },
            { type: 'OPERATOR', value: '(' },
            { type: 'REF', value: '$A1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '123' },
            { type: 'OPERATOR', value: ')' },
            { type: 'OPERATOR', value: '>' },
            { type: 'NUMBER', value: '45.67' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new BasicOperationNode(
            '>',
            new FunctionNode('SUM', [new ReferenceNode('$A1'), new NumberNode(123)]),
            new NumberNode(45.67)
        ));
    });

    test('Parses nested function calls correctly', () => {
        const tokens: Token[] = [
            { type: 'FUNCTION', value: 'MAX' },
            { type: 'OPERATOR', value: '(' },
            { type: 'FUNCTION', value: 'MIN' },
            { type: 'OPERATOR', value: '(' },
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: ')' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '3' },
            { type: 'OPERATOR', value: ')' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new FunctionNode('MAX', [
            new FunctionNode('MIN', [new NumberNode(1), new NumberNode(2)]),
            new NumberNode(3)
        ]));
    });

    test('Handles parentheses correctly', () => {
        const tokens: Token[] = [
            { type: 'OPERATOR', value: '(' },
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: '+' },
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: ')' },
            { type: 'OPERATOR', value: '*' },
            { type: 'NUMBER', value: '3' }
        ];
        const parser = new Parser(tokens);
        const ast = parser.parse();
        expect(ast).toEqual(new BasicOperationNode(
            '*',
            new BasicOperationNode('+', new NumberNode(1), new NumberNode(2)),
            new NumberNode(3)
        ));
    });

    // Negative Tests
    test('Throws error on unexpected token', () => {
        const tokens: Token[] = [
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: '+' },
            { type: 'CHARACTERS', value: '@' }
        ];
        const parser = new Parser(tokens);
        expect(() => parser.parse()).toThrow('Unexpected token: @');
    });

    test('Throws error on missing closing parenthesis', () => {
        const tokens: Token[] = [
            { type: 'OPERATOR', value: '(' },
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: '+' },
            { type: 'NUMBER', value: '2' }
        ];
        const parser = new Parser(tokens);
        expect(() => parser.parse()).toThrow("Expected ')' to close expression");
    });

    test('Throws error on missing open parenthesis', () => {
        const tokens: Token[] = [
            { type: 'FUNCTION', value: 'SUM' },
            { type: 'NUMBER', value: '1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '2' },
            { type: 'OPERATOR', value: ')' }
        ];
        const parser = new Parser(tokens);
        expect(() => parser.parse()).toThrow("Expected '(' after function name");
    });
});
