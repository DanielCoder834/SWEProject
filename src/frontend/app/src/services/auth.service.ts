import axios from "axios";

const API_URL = "https://localhost:9443/api/v1/";

class AuthService {
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

  logout() {
    localStorage.removeItem("currentUser");
  }

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

  getCurrentUser() {
    const userStr = localStorage.getItem("currentUser");
    if (userStr) return JSON.parse(userStr);
    return null;
  }
}

export default new AuthService();
