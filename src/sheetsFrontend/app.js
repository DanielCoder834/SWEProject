document.addEventListener('DOMContentLoaded', function () {
    var createButton = document.getElementById('create');
    var saveButton = document.getElementById('save');
    var deleteButton = document.getElementById('delete');
    createButton === null || createButton === void 0 ? void 0 : createButton.addEventListener('click', function () {
        console.log('Create new file');
    });
    saveButton === null || saveButton === void 0 ? void 0 : saveButton.addEventListener('click', function () {
        console.log('Save file');
    });
    deleteButton === null || deleteButton === void 0 ? void 0 : deleteButton.addEventListener('click', function () {
        console.log('Delete file');
    });
});
