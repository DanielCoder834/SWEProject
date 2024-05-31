// Represents 3 buttons that listen for a key click event
document.addEventListener('DOMContentLoaded', () => {
    const createButton = document.getElementById('create');
    const saveButton = document.getElementById('save');
    const deleteButton = document.getElementById('delete');

    createButton?.addEventListener('click', () => {
        console.log('Create new file');
    });

    saveButton?.addEventListener('click', () => {
        console.log('Save file');
    });

    deleteButton?.addEventListener('click', () => {
        console.log('Delete file');
    });
});