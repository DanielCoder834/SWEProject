import React, { useState, useEffect } from 'react';
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";
import axios from 'axios';
import AuthService from "../services/auth.service";

type TopMenuProps = {
  onCreateSpreadsheet: (rows: number, columns: number, title: string) => void;
  title: string;
};

// @author Adarsh Jayaram
// Represents a grey bar at the top of the localhost:3000 page to hold all the buttons for modifying a spreadsheet
const TopMenu: React.FC<TopMenuProps> = ({ onCreateSpreadsheet, title }) => {
  const API_URL = "https://localhost:9443/api/v1/";
  const [showFileMenu, setShowFileMenu] = useState(false);
  const [currentUser, setCurrentUser] = useState("");
  const [currentPass, setCurrentPass] = useState("");
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [publishers, setPublishers] = useState<string[]>([]);
  const [selectedUser, setSelectedUser] = useState<string>("");
  const [sheets, setSheets] = useState<string[]>([]); 
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showSheetModal, setShowSheetModal] = useState(false);
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

  useEffect(() => {
    const userString = localStorage.getItem('currentUser'); // This can be string or null
    if (userString) {
      const user = JSON.parse(userString); // Safely parse if not null
      if (user && user.username) {
        setCurrentUser(user.username);
      }
    }
  }, []);

  useEffect(() => {
    const passString = localStorage.getItem('currentPassword'); // This can be string or null
    if (passString) {
      const pass = JSON.parse(passString); // Safely parse if not null
      if (pass && pass.password) {
        setCurrentPass(pass.password);
      }
    }
  }, []);

  // @author Brooklyn Schmidt
  // Fetches the publishers when the component mounts.
  useEffect(() => {
    fetchPublishers();
  });

  // @author Brooklyn Schmidt
  // Fetches publishers from API 
  const fetchPublishers = async () => {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.get(API_URL + "getPublishers", {
        headers: {
          'Authorization': basicAuth,
          'Content-Type': 'application/json'
        }
      })
      if (response.status === 200) {
          const publishers : string[]  = response.data.value.map((item: { publisher: string; }) => item.publisher);
          setPublishers(publishers);
      } else {
        console.error("Failed to fetch publishers");
      }
    } catch (error) {
      console.error("Error occurred while fetching publishers:", error);
    }
  };


  // @author Brooklyn Schmidt
  // Fetches sheets from API
  // Parameter is of type Argument, with the publisher field set to the selected user from the User dropdown.
  const fetchSheets = async () => {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.post(API_URL + "getSheets", {
          publisher: selectedUser,
          sheet: "",
          id: "",
          payload: ""
      },{
        headers: {
          'Authorization': basicAuth,
          'Content-Type': 'application/json'
        }
      })
      if (response.status === 200) {
        if (response.data.value === null) {
          setSheets([""]);
        } else {
          const sheets : string[] = response.data.value.map((item: {sheet: string;}) => item.sheet);
          setSheets(sheets);
        }
      } else {
        console.error("Failed to fetch sheets");
      }
    } catch (error) {
      console.error("Error occurred while fetching sheets:", error);
    }
  };

  // @author Brooklyn Schmidt
  // Fetches the createSheet call from the API
  // Parameter is the name of the sheet the user wants to create
  // Creates a sheet for the User logged in

  const fetchCreate = async (sheetName: string) =>  {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.post(API_URL + "createSheet", {
          publisher: currentUser,
          sheet: sheetName,
          id: "",
          payload: ""
      }, {
        headers: {
          'Authorization': basicAuth,
          'Content-Type': 'application/json'
        }
      })

      if (response.status === 200) {
        if (!response.data.success) {
          console.error("Couldn't create a sheet");
        }
      }
    }
    catch (error) {
      console.error("API Failed");
    }
  }

  // @author Brooklyn Schmidt
  // Fetches the deleteSheets call from the API
  // Parameter is the name of the sheet the user wants to create
  // Deletes a sheet from the currently logged in user.
  const fetchDelete = async (sheetName: string) => {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.post(API_URL + "deleteSheet", {
          publisher: currentUser,
          sheet: sheetName,
          id: "",
          payload: ""
      }, {
        headers: {
          'Authorization': basicAuth,
          'Content-Type': 'application/json'
        }
      })

      if (response.status === 200) {
        if (!response.data.success) {
          console.error("Couldn't create a sheet");
        }
      }
    }
    catch (error) {
      console.error("API Failed");
    }
  }

  
  // @author Brooklyn Schmidt
  // Fetches the updatePublisher call from the API
  // Parameter is the owner of the sheet, the sheet to update, and the list of updates
  // Updates the current sheet to the database
 const fetchUpdatePublisher = async (owner: string, sheetName: string, updates: string) => {
  try {
    const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
    const response = await axios.post(API_URL + "updatePublisher", {
      publisher: owner,
      sheet: sheetName,
      id: "",
      payload: updates, // ENTER PARSER CODE
    }, {
    headers: {
      'Authorization': basicAuth,
      'Content-Type': 'application/json'
    }
  })

      if (response.status === 200) {
        if (!response.data.success) {
          console.error("Couldn't update sheet");
        } 
      }
    }
    catch (error) {
      console.error("API Failed");
    }
  } 

  // @author Brooklyn Schmidt
  // Fetches the updateSubscriber call from the API
  // Parameter is the owner of the sheet, the sheet to update, and the list of updates
  // Updates the current sheet to the database
  const fetchUpdateSubscriber = async (owner: string, sheetName: string, updates: string) => {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.post(API_URL + "updateSubscription", {
        publisher: owner,
        sheet: sheetName,
        id: "",
        payload: updates, // ENTER PARSER CODE
      }, {
      headers: {
        'Authorization': basicAuth,
        'Content-Type': 'application/json'
      }
    })
  
        if (response.status === 200) {
          if (!response.data.success) {
            console.error("Couldn't update sheet");
          } 
        }
      }
      catch (error) {
        console.error("API Failed");
      }
    } 
  
  const fetchGetUpdatesForPublished = async (owner: string, sheetName: string, update_id: string) => {
    try {
      const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
      const response = await axios.post(API_URL + "getUpdatesForPublished", {
        publisher: owner,
        sheet: sheetName,
        id: update_id, // enter ID 
        payload: "", 
      }, {
      headers: {
        'Authorization': basicAuth,
        'Content-Type': 'application/json'
      }
    })
  
        if (response.status === 200) {
          if (!response.data.success) {
            console.error("Couldn't retrieve update sheet");
          } 
        }
      }
      catch (error) {
        console.error("API Failed");
      }
    } 

    const fetchGetUpdatesForSubscription = async (owner: string, sheetName: string, update_id: string) => {
      try {
        const basicAuth = 'Basic ' + btoa(currentUser + ':' + currentPass);
        const response = await axios.post(API_URL + "getUpdatesForSubscription", {
          publisher: owner,
          sheet: sheetName,
          id: update_id, // enter ID 
          payload: "", 
        }, {
        headers: {
          'Authorization': basicAuth,
          'Content-Type': 'application/json'
        }
      })
    
          if (response.status === 200) {
            if (!response.data.success) {
              console.error("Couldn't retrieve update sheet");
            } 
          }
        }
        catch (error) {
          console.error("API Failed");
        }
      } 

  const handleFileClick = () => {
    setShowFileMenu(!showFileMenu);
    console.log("Menu is set");
  };

  const handleUserClick = () => {
    setShowUserMenu(!showUserMenu);
  };

  const handleSheetClick = () => {
    setShowSheetModal(true);
  }

  const handleCreateClick = () => {
    setShowCreateModal(true);
  };

  const handleOpenClick = () => {
    setShowOpenModal(true);
  };

  const handleSaveClick = () => {
    setShowSaveModal(true);
  };

  // @author Brooklyn Schmidt
  // When clicking the delete button, sets the delete modal to true
  const handleDeleteClick = () => {
    setShowDeleteModal(true);
  }

  // @author Brooklyn Schmidt
  // When a user selects a user from the list of users, sets the selected user variable to that.
  const handleSaveSelectedUser = async (user: string) => {
    // Set the selected user
    setSelectedUser(user);
    
    // Wait for the selectedUser state to be updated before fetching sheets
    await new Promise(resolve => setTimeout(resolve, 0));
    
    // Fetch sheets for the selected user
    fetchSheets();
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
    fetchCreate(values.title);
    setShowCreateModal(false); // Close the modal after submission
  };

  const handleFileSubmit = (values: { filename: string }) => {
    console.log("File action with filename:", values.filename);
    setShowOpenModal(false);
    setShowSaveModal(false);
  };

  // @author Brooklyn Schmidt
  // When clicking the submit button in the delete modal, deletes the sheet using the filename entered and closes the form.
  const handleDeleteSubmit = (values: {filename: string}) => {
    console.log("Deleting file: " + values.filename);
    fetchDelete(values.filename);
    setShowDeleteModal(false);
  }

  const handleSheetSubmit = (values: {filename: string}) => {
     console.log("Switching to sheet");
     if (selectedUser != currentUser) {
      fetchGetUpdatesForSubscription(selectedUser, values.filename, "0"); // use ID correctly
     } else {
      fetchGetUpdatesForPublished(currentUser, values.filename, "0");
     }
   }

  return (
    <div className="top-menu">
      <div className="title-row">
        <div className="title">{title}</div>
        {currentUser && (
          <div className="user-info">
            Logged in as: <strong>{currentUser}</strong>
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
              <li onClick={handleDeleteClick}>Delete</li> 
            </ul>
          )}
        </div>
        {/* @author Brooklyn Schmidt
        Maps the list of publishers to the User dropdown
        Handles the selected user in the dropdown and fetches the sheets of that user */}
        <div className="dropdown">
          <button onClick={handleUserClick}>Users</button>
          {showUserMenu && (
            <ul className="dropdown-content">
            {publishers.map((publisher, index) => (
              <li key={index} onClick={() => {handleSaveSelectedUser(publisher);
                handleSheetClick();
              }}>{publisher}</li>
            ))}
            </ul>
          )}
        </div>
      </div>

      {showSheetModal && (
        <div className="modal-backdrop">
        <div className="form-container">
          <h2>{selectedUser}'s sheets</h2>
        <Formik
          initialValues={fileForm}
          validationSchema={validationSchema}
          onSubmit={handleSheetSubmit}
           >
             {({ errors, touched }) => (
               <Form>
                <div>
                  <label>Select Sheet</label>
                  <Field name="selectedSheet" as="select">
                    {sheets.map(sheet => (
                    <option key={sheet} value={sheet}>
                      {sheet}
                    </option>
                  ))}
                  </Field>
                </div>
                 <button type="submit">Enter</button>
                 <button type="button" onClick={() => setShowSheetModal(false)}>Cancel</button>
               </Form>
             )}
           </Formik>
         </div>
       </div>
     )}

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
      {/*@author Brooklyn Schmidt
      Shows the delete form which allows users to enter the sheet they want to delete.
      On submission, calls the fetchDelete function.*/}
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
