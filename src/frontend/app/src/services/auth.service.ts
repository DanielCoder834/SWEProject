import axios from "axios";

const API_URL = "http://localhost:9443/api/";

class AuthService {
  login(username: string, password: string) {
    return axios.post(API_URL + "login", { username, password })
      .then(response => {
        if (response.data.accessToken) {
          localStorage.setItem("currentUser", JSON.stringify(response.data));
        }
        return response.data;
      }).catch(error => {
        console.error("Login error:", error.response ? error.response.data : 'No response');
        throw error;
      });
  }

  logout() {
    localStorage.removeItem("currentUser");
  }

  register(username: string, password: string) {
    return axios.post(API_URL + "register", { username, password })
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
