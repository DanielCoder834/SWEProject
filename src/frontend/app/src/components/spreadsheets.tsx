import React, { useEffect, useState } from "react";
import Cell from "./cell";

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