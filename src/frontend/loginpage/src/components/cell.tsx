import React from 'react';

// Type definitions for the props
type CellProps = {
    row: number;
    col: number;
    data: {
        gridData: string[][];
        setGridData: (newData: string[][]) => void;
    };
};

/*
  Represents a single cell in a spreadsheet.
*/
const Cell: React.FC<CellProps> = ({ row, col, data }) => {
    const { gridData, setGridData } = data;

    // Correct event type for the input change event
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newData = [...gridData];
        newData[row][col] = e.target.value;
        setGridData(newData);
    };

    return (
        <div className="cell">
            <input 
                type="text" 
                value={gridData[row][col] || ''} // Added fallback to an empty string
                onChange={handleChange}
                className="cell-input"
            />
        </div>
    );
}

export default Cell;
