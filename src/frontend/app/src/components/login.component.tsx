// @author Adarsh Jayaram plus inspirations from https://www.bezkoder.com/react-typescript-login-example/
import React, { Component } from "react";
import { Formik, Field, Form, ErrorMessage, FormikHelpers } from "formik";
import * as Yup from "yup";
import AuthService from "../services/auth.service"; // AuthService handles the API calls for authentication

// TypeScript type definitions for props and state
type Props = {};

type State = {
  username: string;
  password: string;
  loading: boolean;
  message: string;
  successful: boolean;
};

export default class Login extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.handleLogin = this.handleLogin.bind(this);

    // Initialize state with empty credentials, no loading, no messages, and not successful
    this.state = {
      username: "",
      password: "",
      loading: false,
      message: "",
      successful: false
    };
  }

  // Formik validation schema using Yup for validating the input fields
  validationSchema = Yup.object().shape({
    username: Yup.string().required("This field is required!"),
    password: Yup.string().required("This field is required!"),
  });

  // Event handler for form submission
  handleLogin = (values: { username: string; password: string }, { setSubmitting }: FormikHelpers<{ username: string; password: string }>) => {
    const { username, password } = values;

    // Set loading true to show loading indicator and clear any previous messages
    this.setState({ message: "", loading: true });

    // Call AuthService to perform login, handle response or error
    AuthService.login(username, password).then(
      data => {
        // On success, update the state to reflect the successful login
        this.setState({
          successful: true,
          message: "Login successful!",
          loading: false
        });
        setSubmitting(false); // Reset Formik's submitting state
      },
      error => {
        // Handle errors by setting state with the error message
        const resMessage =
          (error.response &&
            error.response.data &&
            error.response.data.message) ||
          error.message ||
          error.toString();

        this.setState({
          successful: false,
          message: resMessage,
          loading: false
        });
        setSubmitting(false); // Reset Formik's submitting state
      }
    );
  };

  // Render the component UI
  render() {
    const { loading, message, successful } = this.state;

    // Initial values for Formik
    const initialValues = {
      username: "",
      password: "",
    };

    return (
      <div className="col-md-12">
        <div className="card card-container">
          {/* Formik component to handle form with initial values and validation */}
          <Formik
            initialValues={initialValues}
            validationSchema={this.validationSchema}
            onSubmit={this.handleLogin}
          >
            {/* Formik Form component */}
            <Form>
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
                <button type="submit" className="btn btn-primary btn-block" disabled={loading}>
                  {loading && (
                    <span className="spinner-border spinner-border-sm"></span>
                  )}
                  Login
                </button>
              </div>

              {/* Conditionally rendered message */}
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
