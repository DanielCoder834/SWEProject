// to be reviewed
// import '@testing-library/jest-dom';
// import { fireEvent, screen } from '@testing-library/dom';
// import userEvent from '@testing-library/user-event';

// // Mock HTML structure
// document.body.innerHTML = `
// <div class="dropdown">
//   <button class="dropbtn">File</button>
//   <div class="dropdown-content" style="display: none;">
//     <a href="#" id="create">Create</a>
//     <a href="#" id="save">Save</a>
//     <a href="#" id="delete">Delete</a>
//   </div>
// </div>
// `;

// require('./app');  // Assuming the logic for dropdown is in 'app.ts'

// describe('Dropdown functionality', () => {
//   test('Dropdown opens on click', () => {
//     const fileButton = screen.getByText('File');
//     fireEvent.click(fileButton);
//     const dropdownContent = screen.getByText('Create').parentElement;
//     expect(dropdownContent).not.toHaveStyle('display: none');
//   });

//   test('Clicking "Create" logs to the console', () => {
//     const consoleSpy = jest.spyOn(console, 'log');
//     const createButton = screen.getByText('Create');
//     userEvent.click(createButton);
//     expect(consoleSpy).toHaveBeenCalledWith('Create new file');
//     consoleSpy.mockRestore();
//   });

//   test('Clicking "Save" logs to the console', () => {
//     const consoleSpy = jest.spyOn(console, 'log');
//     const saveButton = screen.getByText('Save');
//     userEvent.click(saveButton);
//     expect(consoleSpy).toHaveBeenCalledWith('Save file');
//     consoleSpy.mockRestore();
//   });

//   test('Clicking "Delete" logs to the console', () => {
//     const consoleSpy = jest.spyOn(console, 'log');
//     const deleteButton = screen.getByText('Delete');
//     userEvent.click(deleteButton);
//     expect(consoleSpy).toHaveBeenCalledWith('Delete file');
//     consoleSpy.mockRestore();
//   });
// });
