import React from "react";

/*
  Represents a single cell in a spreadsheet.
*/
const Cell = ({row, col, data}) => {
	const { gridData, setGridData } = data;

	const handleChange = (e) => {
		const newData = [...gridData];
		newData[row][col] = e.target.value;
		setGridData(newData);
	};

	return (
		<>
			<div className="cell">
				<input 
					type="text" 
					value={gridData[row][col]} 
					onChange={handleChange} 
					className="cell-input" 
				/>
			</div>
		</>
	);
}

export default Cell;