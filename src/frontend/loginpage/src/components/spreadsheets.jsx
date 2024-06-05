import React, { useState } from 'react';
import Cell from './cell';

// Define the dimensions of the spreadsheet
const ROWS = 10;
const COLUMNS = 20;

const Spreadsheet = () => {
  // Initialize the grid data
  const initialGridData = Array.from({ length: ROWS }, () => 
    Array.from({ length: COLUMNS }, () => '')
  );

  // State to hold the grid data
  const [gridData, setGridData] = useState(initialGridData);
  
  return (
    <div className="spreadsheet">
      {gridData.map((row, rowIndex) => (
        <div key={rowIndex} className="row">
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

