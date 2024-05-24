// Defined a user to clean prior bug
interface IUser {
  username: string;
  password: string;
}

export default function authHeader() {
  const credentialsStr = localStorage.getItem("userCredentials");
  let credentials: IUser | null = null;

  if (credentialsStr) {
    credentials = JSON.parse(credentialsStr) as IUser;
  }

  if (credentials && credentials.username && credentials.password) {
    // Encode username and password in base64
    const base64Credentials = btoa(
      credentials.username + ":" + credentials.password
    );
    return { Authorization: "Basic " + base64Credentials };
  } else {
    return {}; // Returning an empty object when no credentials are found
  }
}
