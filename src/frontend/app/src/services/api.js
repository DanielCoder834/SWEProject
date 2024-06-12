import axios from 'axios';
import https from 'https';

const api = axios.create({
  baseURL: 'https://localhost:9443/api/v1',
  headers: {
    'Content-Type': 'application/json',
  },
  httpsAgent: new https.Agent({
    rejectUnauthorized: false // Only for development!
  })
});

export const registerUser = async (username, password) => {
  try {
    const response = await api.post('/register', { username, password });
    return response.data;
  } catch (error) {
    console.error("Failed to register:", error.response ? error.response.data : error.message);
    throw error; // Re-throw the error for further handling if necessary
  }
};

export const loginUser = async (username, password) => {
  try {
    const response = await api.post('/login', { username, password });
    return response.data;
  } catch (error) {
    console.error("Failed to login:", error.response ? error.response.data : error.message);
    throw error;
  }
};

export default api;
