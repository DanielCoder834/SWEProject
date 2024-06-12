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
            const parser = new Parser(newData.substring(1));
            const tokens = parser.getTokens();
            const result = evaluateOperation(tokens); // THE ERROR IS HEREzz
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
        const [x, op, y] = tokens;
 
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
 
    const evaluateFunction = (functionToken: Token, argsTokens: Token[]): string => {
        const funcName = functionToken.val.split('(')[0].toUpperCase();
        const args = handleFunctionArgs(functionToken.val);
   
        switch (funcName) {
            case 'IF':
                if (args.length !== 3) return "ERROR";
                const condition = parseFloat(args[0]);
                const trueValue = parseFloat(args[1]);
                const falseValue = parseFloat(args[2]);
                return !isNaN(condition) ? (condition !== 0 ? trueValue.toString() : falseValue.toString()) : "ERROR";
            case 'SUM':
                if (args.every(arg => !isNaN(parseFloat(arg)))) {
                    const sum = args.reduce((acc, curr) => acc + parseFloat(curr), 0);
                    return sum.toString();
                }
                return "ERROR";
            case 'MIN':
                if (args.every(arg => !isNaN(parseFloat(arg)))) {
                    const min = Math.min(...args.map(Number));
                    return min.toString();
                }
                return "ERROR";
            case 'MAX':
                if (args.every(arg => !isNaN(parseFloat(arg)))) {
                    const max = Math.max(...args.map(Number));
                    return max.toString();
                }
                return "ERROR";
            case 'AVG':
                if (args.every(arg => !isNaN(parseFloat(arg)))) {
                    const avg = args.reduce((acc, curr) => acc + parseFloat(curr), 0) / args.length;
                    return avg.toString();
                }
                return "ERROR";
            case 'CONCAT':
                return args.join('');
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