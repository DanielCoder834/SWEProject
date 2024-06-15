import Cell from '../components/cell';
import { render, fireEvent, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';

// @author Alvin Wong
// Tests the methods in cell.tsx
// Each test is labeled with the specific functionality it checks for.
describe('Cell Tests', () => {

    // Positive Tests
    test('Renders correctly with initial data', () => {
        render(<Cell data="Hello" />);
        const inputElement = screen.getByDisplayValue("Hello");
        expect(inputElement).toBeInTheDocument();
    });

    test('Updates the cell with plain text', () => {
        render(<Cell data="Hello" />);
        const inputElement = screen.getByDisplayValue("Hello");
        fireEvent.change(inputElement, { target: { value: "World" } });
        expect(inputElement).toHaveValue("World");
    });

    test('Evaluates an expression and updates the cell with the result', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=1+1" } });
        expect(inputElement).toHaveValue("2");
    });
    
    // Negative Tests
    test('Displays an error message for an invalid expression', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=@" } });
        expect(inputElement).toHaveValue("ERROR: Unknown character: @");
    });

    test('Displays an error message for empty node', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=" } });
        expect(inputElement).toHaveValue("ERROR: Unexpected token: EOF");
    });

    test('Displays an error message for left NaN', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=.+1" } });
        expect(inputElement).toHaveValue("ERROR: Invalid operation");
    });

    test('Displays an error message for right NaN', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=1+." } });
        expect(inputElement).toHaveValue("ERROR: Invalid operation");
    });

    test('Displays an error message for division by 0', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=1/0" } });
        expect(inputElement).toHaveValue("ERROR: Division by zero");
    });

    test('Displays an error message for unsupported operator', () => {
        render(<Cell data="1" />);
        const inputElement = screen.getByDisplayValue("1");
        fireEvent.change(inputElement, { target: { value: "=1=1" } });
        expect(inputElement).toHaveValue("ERROR: Unsupported operator");
    });
});
