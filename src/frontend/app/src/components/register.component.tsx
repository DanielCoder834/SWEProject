// @author Adarsh Jayaram plus inspirations from https://www.bezkoder.com/react-typescript-login-example/
import React, { Component } from "react";
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";
import AuthService from "../services/auth.service"; // Importing the authentication service

type Props = {}; // Empty Props type, no props expected for this component

type State = {
  username: string;
  password: string;
  successful: boolean;
  message: string;
};

export default class Register extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.handleRegister = this.handleRegister.bind(this);

    // Initial state setup with empty credentials and flags for form status
    this.state = {
      username: "",
      password: "",
      successful: false,
      message: "",
    };
  }

  // Validation schema using Yup to enforce input rules for username and password
  validationSchema() {
    return Yup.object().shape({
      username: Yup.string()
        .min(3, "The username must be at least 3 characters long")
        .max(20, "The username cannot be more than 20 characters long")
        .required("This field is required!"),
      password: Yup.string()
        .min(6, "The password must be at least 6 characters long")
        .max(40, "The password cannot be more than 40 characters long")
        .required("This field is required!"),
    });
  }

  // Handler for the registration process
  handleRegister(formValue: { username: string; password: string }) {
    const { username, password } = formValue;
  
    // Resetting state before the API call
    this.setState({ message: "", successful: false });
  
    // Call the AuthService to register the user
    AuthService.register(username, password).then(
      response => {
        // Handle success by updating state to reflect successful registration
        this.setState({
          message: response.message,
          successful: true,
        });
      },
      error => {
        // Handle errors by setting state with the error message received
        this.setState({
          successful: false,
          message: error.message || "Some error occurred."
        });
      }
    );
  }

  render() {
    const { successful, message } = this.state;
    const initialValues = {
      username: "",
      password: "",
    };

    return (
      <div className="col-md-12">
        <div className="card card-container">
          {/* Formik used for form handling with initial values and validation */}
          <Formik
            initialValues={initialValues}
            validationSchema={this.validationSchema()}
            onSubmit={this.handleRegister}
          >
            <Form>
              {/* Conditionally render form fields only if registration was not successful */}
              {!successful && (
                <div>
                  <div className="form-group">
                    <label htmlFor="username">Username</label>
                    <Field name="username" type="text" className="form-control" />
                    <ErrorMessage name="username" component="div" className="alert alert-danger" />
                  </div>

                  <div className="form-group">
                    <label htmlFor="password">Password</label>
                    <Field name="password" type="password" className="form-control" />
                    <ErrorMessage name="password" component="div" className="alert alert-danger" />
                  </div>

                  <div className="form-group">
                    <button type="submit" className="btn btn-primary btn-block">Sign Up</button>
                  </div>
                </div>
              )}

              {/* Message display area, showing success or error messages */}
              {message && (
                <div className="form-group">
                  <div className={successful ? "alert alert-success" : "alert alert-danger"} role="alert">
                    {message}
                  </div>
                </div>
              )}
            </Form>
          </Formik>
        </div>
      </div>
    );
  }
}
