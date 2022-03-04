# File client description
Create a client connection to the server with the port, and then if the connection is established then send data to the server. 
If the data is not reached to the client there may be connection lost between the client and server.
Otherwise the connection between the client and server is not established properly. 
Once the data is sent to the server and it has reached properly then we can terminate the connection between the client and server.

# File Server 
- Please note that the file client will be cloned within the Docker image from a separate repository [here](https://github.com/Shubham-Shingate/file-client)
## Commands to Build and Run Project
- Please note that the Dockerfile within this repository (File Server) will copy the contents from the *second* repository (File Client) into the created Docker image.
### Build Docker Image
Terminal$ `sudo docker build -t [tagname] .`
- Note, change 'tagname' to whatever you want to name it

### Run Docker Image
Terminal$ `sudo docker run -it [tagname]`