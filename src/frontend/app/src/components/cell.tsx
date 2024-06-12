import React from "react";
import Parser, { Token } from "./parser";
import { useState } from "react";
 
// Type definitions for the props
type CellProps = {
    row: number;
    col: number;
    data: {
        gridData: string[][];
        setGridData: (newData: string[][]) => void;
    };
};
 
 
// Represents a single cell in a spreadsheet.
 
const Cell: React.FC<CellProps> = ({ row, col, data }) => {
    const { gridData, setGridData } = data;

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newData = e.target.value;
    
        if (newData.startsWith("=")) {
            const expression = newData.substring(1).trim();
            if (!expression) {
                updateCell("ERROR: No expression provided");
                return;
            }
    
            const parser = new Parser(expression);
            const tokens = parser.getTokens();
    
            if (!tokens.length) {
                updateCell("ERROR: Invalid tokens");
                return;
            }
    
            const result = evaluateOperation(tokens);
            updateCell(result.toString());
        } else {
            updateCell(newData);
        }
    };
    
 
    const updateCell = (val: string) => {
        const newData = [...gridData];
        newData[row][col] = val;
        setGridData(newData);
    };
 
    const evaluateOperation = (tokens: Token[]) => {
        if (tokens.length < 3) {
            console.error("Not enough tokens for operation");
            return "ERROR";
        }
    
        const [x, op, y] = tokens;
    
        // Ensure all necessary tokens are present and of correct type
        if (!op || !x || !y || op.type !== "OPERATOR" || x.type !== "NUMBER" || y.type !== "NUMBER") {
            console.error("Invalid tokens or types", { x, op, y });
            return "ERROR";
        }
    
        const numX = parseFloat(x.val);
        const numY = parseFloat(y.val);
    
        if (isNaN(numX) || isNaN(numY)) {
            console.error(`Invalid number conversion: numX=${numX}, numY=${numY}`);
            return "ERROR: Invalid numbers";
        }
    
        // Perform operations based on operator
        switch (op.val) {
            case "+":
                return numX + numY;
            case "-":
                return numX - numY;
            case "*":
                return numX * numY;
            case "/":
                if (numY === 0) {
                    return "ERROR: Division by zero";
                }
                return numX / numY;
            default:
                console.error("Unsupported operator", op.val);
                return "ERROR: Unsupported operator";
        }
    };
    
    
 
    // Checks if an input is a valid REF.
    const isRef = (val: Token) => {
        return /\$[A-Za-z]+[1-9]\d*/.test(val.val);
    };
 
    // Checks if input REF x comes before input REF y.
    const compareRefs = (x: Token, y: Token): number => {
        const col1Match = x.val.match(/[A-Za-z]+/);
        const row1Match = x.val.match(/\d+/);
        const col2Match = y.val.match(/[A-Za-z]+/);
        const row2Match = y.val.match(/\d+/);
    
        if (!col1Match || !row1Match || !col2Match || !row2Match) {
            return 0; // Return 0 or throw an error if regex matching fails
        }
    
        const [col1] = col1Match;
        const [col2] = col2Match;
        const row1 = parseInt(row1Match[0], 10);
        const row2 = parseInt(row2Match[0], 10);
    
        if (col1 === col2) {
            return row1 - row2;
        }
    
        return col1.localeCompare(col2);
    };
 
    const evaluateFunction = (tokens: Token[]) => {
        if (tokens.length < 3) return "ERROR"; // Ensure there are enough tokens
    
        const [x, op, y] = tokens;
    
        if (!op || !x || !y) return "ERROR"; // Check if any tokens are undefined
    
        switch (op.val) {
            case "+":
                return typeof x === "number" && typeof y === "number" ? x + y : "ERROR";
            case "-":
                return typeof x === "number" && typeof y === "number" ? x - y : "ERROR";
            case "*":
                return typeof x === "number" && typeof y === "number" ? x * y : "ERROR";
            case "/":
                return typeof x === "number" && typeof y === "number" && y !== 0 ? x / y : "ERROR";
            case "<":
                return typeof x === "number" && typeof y === "number" ? (x < y ? 1 : 0) : "ERROR";
            case ">":
                return typeof x === "number" && typeof y === "number" ? (x > y ? 1 : 0) : "ERROR";
            case "=":
                if (typeof x === "number" && typeof y === "number") {
                    return x === y ? 1 : 0;
                } else if (typeof x === "string" && typeof y === "string") {
                    return x === y ? 1 : 0;
                } else {
                    return "ERROR";
                }
            case "<>":
                if (typeof x === "number" && typeof y === "number") {
                    return x !== y ? 1 : 0;
                } else if (typeof x === "string" && typeof y === "string") {
                    return x !== y ? 1 : 0;
                } else {
                    return "ERROR";
                }
            case "&":
                return typeof x === "number" && typeof y === "number" ? (x !== 0 && y !== 0 ? 1 : 0) : "ERROR";
            case "|":
                return typeof x === "number" && typeof y === "number" ? (x !== 0 || y !== 0 ? 1 : 0) : "ERROR";
            case ":":
                return isRef(x) && isRef(y) && compareRefs(x, y) <= 0 ? `${x}:${y}` : "ERROR";
            default:
                return "ERROR";
        }
    };
    
 
    const handleFunctionArgs = (funcString: string): string[] => {
        const argsString = funcString.slice(funcString.indexOf('(') + 1, -1);
        return argsString.split(',').map(arg => arg.trim());
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