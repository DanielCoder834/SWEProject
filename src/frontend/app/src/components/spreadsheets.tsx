import React, { useState, useEffect } from 'react';
import Cell from './cell';

// Define the props type for the Spreadsheet component.
type SpreadsheetProps = {
  dimensions: { rows: number; columns: number };
};

// Spreadsheet component that dynamically creates a grid of cells.
const Spreadsheet: React.FC<SpreadsheetProps> = ({ dimensions }) => {
  const { rows, columns } = dimensions;

  // Function to create an initial grid of empty strings based on the given dimensions.
  const createInitialGrid = () => {
    return Array.from({ length: rows }, () => Array.from({ length: columns }, () => ""));
  };

  // State to hold the data for each cell in the grid, initialized with createInitialGrid.
  const [gridData, setGridData] = useState(createInitialGrid());

  // Effect hook to reset the grid when the dimensions change.
  useEffect(() => {
    setGridData(createInitialGrid());
  }, [rows, columns]);

  // Function to convert a column index into a spreadsheet column label (e.g., 0 -> A, 1 -> B, etc.).
  const numToCol = (index: number) => {
    let label = '';
    let alphaIndex = index;
    while (alphaIndex >= 0) {
      label = String.fromCharCode((alphaIndex % 26) + 65) + label;
      alphaIndex = Math.floor(alphaIndex / 26) - 1;
    }
    return label;
  };

  return (
    <div className="spreadsheet-container">
      {/* Render a row of column headers, converting column indices to labels. */}
      <div className="row colLabel">
        {Array.from({ length: columns }, (_, index) => (
          <div key={`header-${index}`} className="cell header">
            {numToCol(index)}
          </div>
        ))}
      </div>
      {/* Render the grid rows, each containing cells. */}
      {gridData.map((row, rowIndex) => (
        <div key={rowIndex} className="row">
          <div className="rowLabel">{rowIndex + 1}</div>
          {row.map((_, colIndex) => (
            <Cell
              key={`${rowIndex}-${colIndex}`}
              row={rowIndex}
              col={colIndex}
              data={{ gridData, setGridData }}
            />
          ))}
        </div>
      ))}
    </div>
  );
};

export default Spreadsheet;