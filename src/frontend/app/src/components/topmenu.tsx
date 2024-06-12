import React, { useState, useEffect } from 'react';
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";

type TopMenuProps = {
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
  title: string;
};

const TopMenu: React.FC<TopMenuProps> = ({ onCreateSpreadsheet, title }) => {
  const [showFileMenu, setShowFileMenu] = useState(false);
  const [currentUser, setCurrentUser] = useState<string>("");
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [publishers, setPublishers] = useState<string[]>([]);
  const [selectedUser, setSelectedUser] = useState<string>("");
  const [sheets, setSheets] = useState<string[]>([]); 
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showOpenModal, setShowOpenModal] = useState(false);
  const [showSaveModal, setShowSaveModal] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [createForm] = useState({
    title: "",
    rows: 0,
    columns: 0
  });
  const [fileForm] = useState({
    filename: ""
  });

  // Written by Brooklyn Schmidt
  useEffect(() => {
    // Fetch publishers when component mounts
    fetchPublishers();
  }, []);

  // Written by Brooklyn Schmidt
  const fetchPublishers = async () => {
    try {
      const response = await fetch("https://localhost:9443/api/v1/getPublishers", {
        method: "GET"
      });
      if (response.ok) {
        const data = await response.json();

        if (data.success) {
          const publishers : string[]  = data.value.map((item: { publisher: string; }) => item.publisher);
          setPublishers(publishers);
        }
        else {
          console.error("API call failed");
        }
      } else {
        console.error("Failed to fetch publishers");
      }
    } catch (error) {
      console.error("Error occurred while fetching publishers:", error);
    }
  };


  // Written by Brooklyn Schmidt
  const fetchSheets = async () => {
    try {
      const argument = {
        publisher: selectedUser,
        sheet: "",
        id: "",
        payload: ""
      } 
      const response = await fetch("https://localhost:9443/api/v1/getSheets", {
        method: "POST",
        body: JSON.stringify({argument})
      });
      if (response.ok) {
        const data = await response.json();

        if (data.success) {
          const sheets : string[] = data.value.map((item: {sheet: string;}) => item.sheet);
          setSheets(sheets);
        }
        else {
          console.error("API fetch failed");
        }
      } else {
        console.error("Failed to fetch sheets");
      }
    } catch (error) {
      console.error("Error occurred while fetching sheets:", error);
    }
  };

  // Written by Brooklyn Schmidt
  const fetchCreate = async (sheetName: string) =>  {
    try {
      const argument =  {
        publisher: currentUser, // fix
        sheet: sheetName,
        id: "",
        payload: ""
      }

      const response = await fetch("https://localhost:9443/api/v1/createSheet",
        {
          method: "POST",
          body: JSON.stringify(argument)
        }
      );

      if (response.ok) {
        const data = await response.json();

        if (!data.success) {
          console.error("Couldn't create a sheet");
        }
      }
    }
    catch (error) {
      console.error("API Failed");
    }
  }

  // Written by Brooklyn Schmidt 
  // can refactor 
  const fetchDelete = async (sheetName: string) => {
    try {
      const argument =  {
        publisher: currentUser, // fix
        sheet: sheetName,
        id: "",
        payload: ""
      }

      const response = await fetch("https://localhost:9443/api/v1/deleteSheet",
        {
          method: "POST",
          body: JSON.stringify(argument)
        }
      );

      if (response.ok) {
        const data = await response.json();

        if (!data.success) {
          console.error("Couldn't create a sheet");
        }
      }
    }
    catch (error) {
      console.error("API Failed");
    }
  }

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

  // Written by Brooklyn Schmidt
  const handleDeleteClick = () => {
    setShowDeleteModal(true);
  }

  // written by Brooklyn Schmidt
  const handleSaveSelectedUser = (user: string) => {
    setSelectedUser(user);
  }


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
    fetchCreate(values.title);
    setShowCreateModal(false); // Close the modal after submission
  };

  const handleFileSubmit = (values: { filename: string }) => {
    console.log("File action with filename:", values.filename);
    setShowOpenModal(false);
    setShowSaveModal(false);
  };

  const handleDeleteSubmit = (values: {filename: string}) => {
    console.log("Deleting file: " + values.filename);
    fetchDelete(values.filename);
    setShowDeleteModal(false);
  }

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
              <li onClick={handleOpenClick}>Open</li>
              <li onClick={handleSaveClick}>Save</li>
              <li onClick={handleDeleteClick}>Delete</li> 
            </ul>
          )}
        </div>
        {/* Written by Brooklyn Schmidt */}
        <div className="dropdown-users">
        <button onClick={handleUserClick}>Users</button>
        {showUserMenu && (
          <ul className="dropdown-content">
          {publishers.map((publisher, index) => (
            <li key={index} onClick={() => {handleSaveSelectedUser(publisher);
              fetchSheets();
            }}>{publisher}</li>
          ))}
          </ul>
        )}
        </div>
        <div className="dropdown-sheets">
          <button onClick={handleUserClick}>Sheets</button>
          {showUserMenu &&
            <ul className="dropdown-content-sheets">
              {sheets.map((sheet, index) => (
                <li key={index} onClick={() => handleUserClick()}>{sheet}</li>
              ))}
            </ul>
          } 
        </div>
      </div>

      {showCreateModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={createForm}
              validationSchema={validationSchema}
              onSubmit={handleCreateSubmit}
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

      {showOpenModal && (
        <div className="modal-backdrop">
          <div className="form-container">
            <Formik
              initialValues={fileForm}
              validationSchema={fileValidationSchema}
              onSubmit={handleFileSubmit}
            >
              {({ errors, touched }) => (
                <Form>
                  <div>
                    <label>Filename</label>
                    <Field name="filename" type="text" />
                    <ErrorMessage name="filename" component="div" className="alert alert-danger" />
                  </div>
                  <button type="submit">Enter</button>
                  <button type="button" onClick={() => setShowDeleteModal(false)}>Cancel</button>
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
              initialValues={fileForm}
              validationSchema={fileValidationSchema}
              onSubmit={handleFileSubmit}
            >
              {({ errors, touched }) => (
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

{"Written by Brooklyn Schmidt"}
      {showDeleteModal && (
              <div className="modal-backdrop">
                <div className="form-container">
                  <Formik
                    initialValues={fileForm}
                    validationSchema={fileValidationSchema}
                    onSubmit={handleDeleteSubmit}
                  >
                    {({ errors, touched }) => (
                      <Form>
                        <div>
                          <label>Filename</label>
                          <Field name="filename" type="text" />
                          <ErrorMessage name="filename" component="div" className="alert alert-danger" />
                        </div>
                        <button type="submit">Enter</button>
                        <button type="button" onClick={() => setShowDeleteModal(false)}>Cancel</button>
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
