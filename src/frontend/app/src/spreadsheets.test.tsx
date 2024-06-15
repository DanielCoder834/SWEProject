import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect'; // for better assertions
import Spreadsheet from '../components/spreadsheets';

// Mock the Cell component
jest.mock('../components/cell', () => ({ data }: { data: string }) => <input data-testid="cell" value={data} readOnly />);

// @author Alvin Wong
// Tests the methods in spreadsheets.tsx
// Each test is labeled with the specific functionality it checks for.
describe('Spreadsheet', () => {
  test('Renders the correct number of rows and columns', () => {
    const dimensions = { rows: 3, columns: 4 };
    render(<Spreadsheet dimensions={dimensions} />);

    // Check if column headers are correct
    const colHeaders = screen.getAllByText(/^[A-Z]+$/);
    expect(colHeaders).toHaveLength(dimensions.columns);
    expect(colHeaders.map(col => col.textContent)).toEqual(['A', 'B', 'C', 'D']);

    // Check if row labels are correct
    const rowLabels = screen.getAllByText(/^[1-9][0-9]*$/);
    expect(rowLabels).toHaveLength(dimensions.rows);
    expect(rowLabels.map(row => row.textContent)).toEqual(['1', '2', '3']);

    // Check if the grid cells are rendered correctly
    const cells = screen.getAllByTestId('cell');
    expect(cells).toHaveLength(dimensions.rows * dimensions.columns);
  });

  test('Resets the grid when dimensions change', () => {
    const { rerender } = render(<Spreadsheet dimensions={{ rows: 2, columns: 2 }} />);
    
    let cells = screen.getAllByTestId('cell');
    expect(cells).toHaveLength(4);

    rerender(<Spreadsheet dimensions={{ rows: 3, columns: 3 }} />);

    cells = screen.getAllByTestId('cell');
    expect(cells).toHaveLength(9);
  });
});
