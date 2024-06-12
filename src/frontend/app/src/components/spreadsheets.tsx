import React, { useState, useEffect } from 'react';
import Cell from './cell';

// Handles dimensions of the spreadsheet
type SpreadsheetProps = {
  dimensions: { rows: number; columns: number };
};

const Spreadsheet: React.FC<SpreadsheetProps> = ({ dimensions }) => {
  const { rows, columns } = dimensions;

  const createInitialGrid = () => {
    return Array.from({ length: rows }, () => Array.from({ length: columns }, () => ""));
  };

  const [gridData, setGridData] = useState(createInitialGrid());

  useEffect(() => {
    setGridData(createInitialGrid()); // Resets the grid when dimensions change
  }, [rows, columns]);

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
      {/* <div className="row colLabel">
        {Array.from({ length: columns }, (_, index) => (
          <div key={`header-${index}`} className="cell header">
            {numToCol(index)}
          </div>
        ))}
      </div> */}
      <div className="colLabel">
        {Array.from({ length: columns }).map((_, rowIndex) => (
          <div key={`header-${rowIndex}`} className="cell header">
              {numToCol(rowIndex)}
            </div>
          ))}
      </div>
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