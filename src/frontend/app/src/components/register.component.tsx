import React, { Component } from "react";
import { Formik, Field, Form, ErrorMessage } from "formik";
import * as Yup from "yup";
import AuthService from "../services/auth.service";
import axios from "axios";


type Props = {};

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

    this.state = {
      username: "",
      password: "",
      successful: false,
      message: "",
    };
  }

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

  handleRegister(formValue: { username: string; password: string }) {
    const { username, password } = formValue;
  
    this.setState({ message: "", successful: false });
  
    AuthService.register(username, password).then(
      response => {
        this.setState({
          message: response.message,
          successful: true,
        });
      },
      error => {
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
          <Formik
            initialValues={initialValues}
            validationSchema={this.validationSchema()}
            onSubmit={this.handleRegister}
          >
            <Form>
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
