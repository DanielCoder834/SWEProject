The various ways to run the code and how to run them.

Disclaimer: Some toots may need to be installed such as Docker, 
llvm-cov, cargo or openssl

Required Libraries/Packages: RustUp, OpenSSL, Postgres-Client and Postgres

Before running the server, make a Copy of the .env.example and fll it with the 
 necessary information and rename it to .env

1. Running the server
   1. Cd into or back to the SWEProject directory
   2. Type 'make server' into the terminal


2. Testing the server 
   1. Cd into or back to the SWEProject directory
   2. Type 'make server-test' into the terminal


3. Testing the server with coverage
   1. Install llvm-coverage - Can be done through brew
   2. Cd into or back to the SWEProject directory
   3. Type 'make server-test-coverage' into the terminal


4. Running the Server with Docker 
   1. Install Docker - There is an application called Docker Desktop you can use
   2. Type 'make server-docker' into the terminal



Other terminal commands (need to be in the backend directory): 
1. For Diesel (ORM):
     - Creating the database: diesel setup
     - Making new tables: diesel migration generate create_<table_name>
