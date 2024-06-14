// @author Alvin Wong
import React from "react";
import { useState } from "react";
import { Parser } from "./parser1";  // Ensure this import is correct
import { Tokenize } from "./tokenize";
import { ASTNode, NumberNode, BasicOperationNode, FunctionNode, ReferenceNode, StringNode } from "./astnodes";

// Type definitions for the props
type CellProps = {
    row: number;
    col: number;
    data: {
        gridData: string[][];
        setGridData: (newData: string[][]) => void;
    };
};

const Cell: React.FC<CellProps> = ({ row, col, data }) => {
    const { gridData, setGridData } = data;

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newData = e.target.value;
        updateCell(newData); // Update the cell immediately with entered data

        if (newData.startsWith("=")) {
            const expression = newData.substring(1).trim();
            try {
                const tokenizer = new Tokenize(expression);
                const tokens = tokenizer.tokenize(); // Tokenize the expression
                const parser = new Parser(tokens); // Initialize the parser with tokens
                const ast = parser.parse(); // Parse the tokens to AST
                const result = evaluateOperation(ast); // Evaluate the AST
                updateCell(result.toString()); // Update the cell with the result of the evaluation
            } catch (error) {
                if (error instanceof Error) {
                    updateCell("ERROR: " + error.message);
                } else {
                    updateCell("ERROR: An unexpected error occurred");
                }
            }
        }
    };

    const updateCell = (val: string) => {
        const newData = [...gridData];
        newData[row][col] = val;
        setGridData(newData);
    };

    const evaluateOperation = (node: ASTNode, context?: any): any => {
        if (!node) return "ERROR: Empty node";
    
        switch (node.type) {
            case "Number":
                const numberNode = node as NumberNode;
                return numberNode.value;
            case "BinaryOperation":
                const binaryNode = node as BasicOperationNode;
                const left = evaluateOperation(binaryNode.left, context);
                const right = evaluateOperation(binaryNode.right, context);
                if (isNaN(left) || isNaN(right)) {
                    return "ERROR: Invalid operation";
                }
                switch (binaryNode.operator) {
                    case "+":
                        return left + right;
                    case "-":
                        return left - right;
                    case "*":
                        return left * right;
                    case "/":
                        return right !== 0 ? left / right : "ERROR: Division by zero";
                    default:
                        return "ERROR: Unsupported operator";
                }
            case "CellReference":
                const cellNode = node as ReferenceNode;
                return evaluateCell(cellNode.reference, context);
            case "FunctionCall":
                const functionNode = node as FunctionNode;
                const args = functionNode.arguments.map(arg => evaluateOperation(arg, context));
                return executeFunction(functionNode.functionName, args);
            case "String":
                const stringNode = node as StringNode;
                return stringNode.value;
            default:
                return "ERROR: Unknown node type";
        }
    };
    
    // @author Adarsh Jayaram 
    const evaluateCell = (reference: string, context: any): any => {
        if (!context || !context.getCell) {
            return "ERROR: Context or getCell method not provided";
        }
        const cellValue = context.getCell(reference); // Retrieve the cell value from the spreadsheet
        if (cellValue && typeof cellValue === 'object' && 'type' in cellValue) {
            return evaluateOperation(cellValue, context); // Recursively evaluate if the cell contains a formula
        }
        return cellValue; // Return the value directly if it's a number or string
    };
    
    // @author Adarsh Jayaram
    const executeFunction = (name: string, args: any[]): any => {
        // You can implement specific functions like SUM, AVERAGE, etc.
        switch (name.toLowerCase()) {
            case "sum":
                return args.reduce((acc, val) => acc + parseFloat(val), 0);
            case "average":
                return args.reduce((acc, val) => acc + parseFloat(val), 0) / args.length;
            default:
                return "ERROR: Function not implemented";
        }
    };

    return (
        <>
            <div className="cell">
                <input
                    type="text"
                    value={gridData[row][col] || ''}
                    onChange={handleChange}
                    className="cell-input"
                />
            </div>
        </>
    );
};

export default Cell;