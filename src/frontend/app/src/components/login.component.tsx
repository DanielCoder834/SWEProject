import React, { Component } from "react";
import { Formik, Field, Form, ErrorMessage, FormikHelpers } from "formik";
import * as Yup from "yup";
import AuthService from "../services/auth.service"; // Make sure AuthService is imported correctly

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

    this.state = {
      username: "",
      password: "",
      loading: false,
      message: "",
      successful: false
    };
  }

  validationSchema = Yup.object().shape({
    username: Yup.string().required("This field is required!"),
    password: Yup.string().required("This field is required!"),
  });

  handleLogin = (values: { username: string; password: string }, { setSubmitting }: FormikHelpers<{ username: string; password: string }>) => {
    const { username, password } = values;

    this.setState({ message: "", loading: true });

    AuthService.login(username, password).then(
      data => {
        this.setState({
          successful: true,
          message: "Login successful!",
          loading: false
        });
        setSubmitting(false);
        // Here you can redirect the user to another page or perform other actions as needed
      },
      error => {
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
        setSubmitting(false);
      }
    );
  };

  render() {
    const { loading, message, successful } = this.state;

    const initialValues = {
      username: "",
      password: "",
    };

    return (
      <div className="col-md-12">
        <div className="card card-container">
          <Formik
            initialValues={initialValues}
            validationSchema={this.validationSchema}
            onSubmit={this.handleLogin}
          >
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
