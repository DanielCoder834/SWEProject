import React, { useEffect, useState } from "react";
import Cell from "./cell";

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
    setGridData(createInitialGrid());
  }, [rows, columns]);

  const numToCol = (index: number) => {
    let label = '';
    while (index >= 0) {
      label = String.fromCharCode((index % 26) + 65) + label;
      index = Math.floor(index / 26) - 1;
    }
    return label;
  };

  return (
    <div className="spreadsheet-container">
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
          {row.map((cellData, colIndex) => (
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
