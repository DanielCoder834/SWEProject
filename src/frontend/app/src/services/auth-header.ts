export default function authHeader() {
    const userStr = localStorage.getItem("user");
    let user = null;
    if (userStr)
      user = JSON.parse(userStr);
  
    if (user && user.username && user.password) {
      return { Authorization: 'Basic ' + Buffer.from(user.username + ':' + user.password) }; // for Spring Boot back-end
    } else {
      return { Authorization: '' }; // for Spring Boot back-end
    }
  }