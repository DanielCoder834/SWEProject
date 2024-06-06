import React, { useState } from "react";
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";

type TopMenuProps = {
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
  title: string;
};

const TopMenu: React.FC<TopMenuProps> = ({ onCreateSpreadsheet, title }) => {
  const [showFileMenu, setShowFileMenu] = useState(false);
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [createForm, setCreateForm] = useState({
    title: "",
    rows: 0,
    columns: 0
  });

  const handleFileClick = () => {
    setShowFileMenu(!showFileMenu);
  };

  const handleUserClick = () => {
    setShowUserMenu(!showUserMenu);
  };

  const handleCreateClick = () => {
    setShowCreateModal(true);
  };

  const validationSchema = Yup.object().shape({
    title: Yup.string().required("Title is required!"),
    rows: Yup.number().min(1, "At least 1 row required").required("Rows are required"),
    columns: Yup.number().min(1, "At least 1 column required").required("Columns are required")
  });

  const handleSubmit = (values: { title: string; rows: number; columns: number }) => {
    onCreateSpreadsheet(values.rows, values.columns, values.title);
    setShowCreateModal(false); // Close the modal after submission
  };

  return (
    <div className="top-menu">
      <div className="title-row">
        <div className="title">{title}</div>
      </div>
      <div className="buttons-row">
        <div className="dropdown">
          <button onClick={handleFileClick}>File</button>
          {showFileMenu && (
            <ul className="dropdown-content">
              <li onClick={handleCreateClick}>Create</li>
              <li onClick={() => console.log("Open Document")}>Open</li>
              <li onClick={() => console.log("Save Document")}>Save</li>
            </ul>
          )}
        </div>
        <div className="dropdown">
          <button onClick={handleUserClick}>Users</button>
          {showUserMenu && (
            <ul className="dropdown-content">
              <li>Bob</li>
              <li>Joe</li>
              <li>Sally</li>
            </ul>
          )}
        </div>
      </div>

      {showCreateModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={createForm}
              validationSchema={validationSchema}
              onSubmit={handleSubmit}
            >
              {({ errors, touched }) => (
                <Form>
                  <div>
                    <label>Title</label>
                    <Field name="title" type="text" />
                    <ErrorMessage name="title" component="div" className="alert alert-danger" />
                  </div>
                  <div>
                    <label>Rows</label>
                    <Field name="rows" type="number" />
                    <ErrorMessage name="rows" component="div" className="alert alert-danger" />
                  </div>
                  <div>
                    <label>Columns</label>
                    <Field name="columns" type="number" />
                    <ErrorMessage name="columns" component="div" className="alert alert-danger" />
                  </div>
                  <button type="submit">Enter</button>
                  <button type="button" onClick={() => setShowCreateModal(false)}>Cancel</button>
                </Form>
              )}
            </Formik>
          </div>
        </div>
      )}
    </div>
  );
};

export default TopMenu;
