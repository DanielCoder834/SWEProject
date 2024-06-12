import React, { useState, useEffect } from "react";
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";
import AuthService from "../services/auth.service";

type TopMenuProps = {
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
  title: string;
};

const TopMenu: React.FC<TopMenuProps> = ({ onCreateSpreadsheet, title }) => {
  const [showFileMenu, setShowFileMenu] = useState(false);
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showOpenModal, setShowOpenModal] = useState(false);
  const [showSaveModal, setShowSaveModal] = useState(false);
  const [username, setUsername] = useState<string | null>(null);
  const [userList, setUserList] = useState<string[]>([]);

  useEffect(() => {
    const storedUsername = localStorage.getItem("username");
    if (storedUsername) {
      setUsername(storedUsername);
    }
  }, []);

  const handleFileClick = () => {
    setShowFileMenu(!showFileMenu);
  };

  const handleUserClick = () => {
    setShowUserMenu(!showUserMenu);
  };

  const handleCreateClick = () => {
    setShowCreateModal(true);
  };

  const handleOpenClick = () => {
    setShowOpenModal(true);
  };

  const handleSaveClick = () => {
    setShowSaveModal(true);
  };

  const validationSchema = Yup.object().shape({
    title: Yup.string().required("Title is required!"),
    rows: Yup.number().min(1, "At least 1 row required").required("Rows are required"),
    columns: Yup.number().min(1, "At least 1 column required").required("Columns are required")
  });

  const fileValidationSchema = Yup.object().shape({
    filename: Yup.string().required("Filename is required!")
  });

  const handleCreateSubmit = (values: { title: string; rows: number; columns: number }) => {
    onCreateSpreadsheet(values.rows, values.columns, values.title);
    setShowCreateModal(false); // Close the modal after submission
  };

  const handleFileSubmit = (values: { filename: string }) => {
    console.log("File action with filename:", values.filename);
    setShowOpenModal(false);
    setShowSaveModal(false);
  };

  return (
    <div className="top-menu">
      <div className="title-row">
        <div className="title">{title}</div>
        {username && (
          <div className="user-info">
            Logged in as: <strong>{username}</strong>
          </div>
        )}
      </div>
      <div className="buttons-row">
        <div className="dropdown">
          <button onClick={handleFileClick}>File</button>
          {showFileMenu && (
            <ul className="dropdown-content">
              <li onClick={handleCreateClick}>Create</li>
              <li onClick={handleOpenClick}>Open</li>
              <li onClick={handleSaveClick}>Save</li>
            </ul>
          )}
        </div>
        <div className="dropdown">
          <button onClick={handleUserClick}>Users</button>
          {showUserMenu && (
            <ul className="dropdown-content">
              {userList.map((user, index) => (
                <li key={index}>{user}</li>
              ))}
            </ul>
          )}
        </div>
      </div>

      {/* Modals for Create, Open, Save */}
      {/* Create Modal */}
      {showCreateModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={{ title: "", rows: 0, columns: 0 }}
              validationSchema={validationSchema}
              onSubmit={handleCreateSubmit}
            >
              {() => (
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

      {/* Open and Save Modals (similar structure) */}
      {showOpenModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={{ filename: "" }}
              validationSchema={fileValidationSchema}
              onSubmit={handleFileSubmit}
            >
              {() => (
                <Form>
                  <div>
                    <label>Filename</label>
                    <Field name="filename" type="text" />
                    <ErrorMessage name="filename" component="div" className="alert alert-danger" />
                  </div>
                  <button type="submit">Enter</button>
                  <button type="button" onClick={() => setShowOpenModal(false)}>Cancel</button>
                </Form>
              )}
            </Formik>
          </div>
        </div>
      )}

      {showSaveModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={{ filename: "" }}
              validationSchema={fileValidationSchema}
              onSubmit={handleFileSubmit}
            >
              {() => (
                <Form>
                  <div>
                    <label>Filename</label>
                    <Field name="filename" type="text" />
                    <ErrorMessage name="filename" component="div" className="alert alert-danger" />
                  </div>
                  <button type="submit">Enter</button>
                  <button type="button" onClick={() => setShowSaveModal(false)}>Cancel</button>
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