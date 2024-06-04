import React from 'react';
import DataGrid from 'react-data-grid';
import '../App.css';  // Adjusted relative path

const columns = [
  { key: 'id', name: 'ID' },
  { key: 'title', name: 'Title' }
];

const rows = [{ id: 1, title: 'Example' }];

function Spreadsheets() {
  return <DataGrid columns={columns} rows={rows} />;
}

export default Spreadsheets;
