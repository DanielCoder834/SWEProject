// @author Adarsh Jayaram plus inspirations from https://github.com/bezkoder/react-typescript-login-example
import React, { Component } from "react";
import { Routes, Route, Link } from "react-router-dom";
import "bootstrap/dist/css/bootstrap.min.css";
import "./App.css";

import AuthService from "./services/auth.service";
import IUser from './types/user.type'; // Ensure this path accurately reflects where IUser is defined

import Login from "./components/login.component";
import Register from "./components/register.component";
import Home from "./components/home.component";
import Profile from "./components/profile.component";
import BoardUser from "./components/board-user.component";
import BoardModerator from "./components/board-moderator.component";
import BoardAdmin from "./components/board-admin.component";

import EventBus from "./common/EventBus";

type Props = {};

type State = {
  showModeratorBoard: boolean,
  showAdminBoard: boolean,
  currentUser: IUser | null, // Adjusted to be nullable
  dimensions: { rows: number, columns: number },
  title: string
}

// Main application component that manages routing and global state.
class App extends Component<Props, State> {
  // Constructor: Sets up the component state and binds methods.
  constructor(props: Props) {
    super(props);
    this.logOut = this.logOut.bind(this);

    // Initial state includes user roles, current user, and default spreadsheet dimensions.
    this.state = {
      showModeratorBoard: false,
      showAdminBoard: false,
      currentUser: null,
      dimensions: { rows: 10, columns: 20 },
      title: "Untitled"
    };
  }

  // componentDidMount: Fetches current user data and sets user permissions.
  componentDidMount() {
    const user = AuthService.getCurrentUser();

    if (user && user.roles) {
      this.setState({
        currentUser: user,
        showModeratorBoard: user.roles.includes("ROLE_MODERATOR"),
        showAdminBoard: user.roles.includes("ROLE_ADMIN"),
      });
    }

    EventBus.on("logout", this.logOut);
  }

  // componentWillUnmount: Cleans up event listeners.
  componentWillUnmount() {
    EventBus.remove("logout", this.logOut);
  }

  // logOut: Logs out the current user and updates the state accordingly.
  logOut() {
    AuthService.logout();
    this.setState({
      showModeratorBoard: false,
      showAdminBoard: false,
      currentUser: null,
    });
  }

  // handleCreateSpreadsheet: Updates state based on the new spreadsheet dimensions and title.
  handleCreateSpreadsheet = (rows: number, columns: number, title: string) => {
    this.setState({
      dimensions: { rows, columns },
      title
    });
  };

  // render: Defines the layout and routing for the application.
  render() {
    const { currentUser, showModeratorBoard, showAdminBoard, dimensions, title } = this.state;

    return (
      <div>
        <nav className="navbar navbar-expand navbar-dark bg-dark">
          <Link to={"/"} className="navbar-brand">
            Husksheets
          </Link>
          <div className="navbar-nav mr-auto">
            <li className="nav-item">
              <Link to={"/home"} className="nav-link">
                Home
              </Link>
            </li>
          </div>
          <div className="navbar-nav ml-auto">
            {currentUser ? (
              <>
                <li className="nav-item">
                  <Link to={"/profile"} className="nav-link">
                    {currentUser.username}
                  </Link>
                </li>
                <li className="nav-item">
                  <a href="/login" className="nav-link" onClick={this.logOut}>
                    LogOut
                  </a>
                </li>
              </>
            ) : (
              <>
                <li className="nav-item">
                  <Link to={"/login"} className="nav-link">
                    Login
                  </Link>
                </li>
                <li className="nav-item">
                  <Link to={"/register"} className="nav-link">
                    Sign Up
                  </Link>
                </li>
              </>
            )}
          </div>
        </nav>

        <div className="container mt-3">
          <Routes>
            <Route path="/" element={<Home dimensions={dimensions} title={title} onCreateSpreadsheet={this.handleCreateSpreadsheet} />} />
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
            <Route path="/profile" element={<Profile />} />
            <Route path="/user" element={<BoardUser />} />
            <Route path="/mod" element={<BoardModerator />} />
            <Route path="/admin" element={<BoardAdmin />} />
          </Routes>
        </div>
      </div>
    );
  }
}

export default App;
