// @author Adarsh Jayaram
import axios from "axios";

const API_URL = "https://localhost:9443/api/v1/";

class AuthService {
  // login: Authenticates the user by sending credentials to the server and stores them in localStorage.
  login(username: string, password: string) {
    const basicAuth = 'Basic ' + btoa(username + ':' + password);
    localStorage.setItem('currentUser', JSON.stringify({ username }));
    localStorage.setItem('currentPassword', JSON.stringify({ password }));
    return axios.get(API_URL + "register", {
      headers: {
        'Authorization': basicAuth,
        'Content-Type': 'application/json'
      }
    })
    .then(response => {
      return response.data;
    }).catch(error => {
      console.error("Registration error:", error.response ? error.response.data : 'No response');
      throw error;
    });
  }

  // logout: Removes the stored user credentials from localStorage.
  logout() {
    localStorage.removeItem("currentUser");
    localStorage.removeItem("currentPassword");  // Ensure to remove password as well if stored
  }

  // register: Registers a new user by sending their credentials to the server and storing them if successful.
  register(username: string, password: string) {
    const basicAuth = 'Basic ' + btoa(username + ':' + password);
    localStorage.setItem('currentUser', JSON.stringify({ username }));
    localStorage.setItem('currentPassword', JSON.stringify({ password }));
    return axios.get(API_URL + "register", {
      headers: {
        'Authorization': basicAuth,
        'Content-Type': 'application/json'
      }
    })
    .then(response => {
      return response.data;
    }).catch(error => {
      console.error("Registration error:", error.response ? error.response.data : 'No response');
      throw error;
    });
  }

  // getCurrentUser: Retrieves the current user's data from localStorage.
  getCurrentUser() {
    const userStr = localStorage.getItem("currentUser");
    if (userStr) return JSON.parse(userStr);
    return null;
  }
}

export default new AuthService();
