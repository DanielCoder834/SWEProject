// @author Adarsh Jayaram plus inspirations from https://www.bezkoder.com/react-typescript-login-example/
// authHeader: Retrieves user credentials from localStorage and constructs an Authorization header.
export default function authHeader() {
  const userStr = localStorage.getItem("user"); // Retrieve the user data from local storage.
  let user = null;
  if (userStr) {
      user = JSON.parse(userStr); // Parse the user data string into an object.
  }

  if (user && user.username && user.password) {
      // If user data is present and includes username and password, create a Basic Auth header.
      return { Authorization: 'Basic ' + Buffer.from(user.username + ':' + user.password).toString('base64') }; // for Spring Boot back-end
  } else {
      // Return an empty Authorization header if no user data is available.
      return { Authorization: '' }; // for Spring Boot back-end
  }
}