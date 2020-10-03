#include <stdio.h>
#include <stdlib.h>

#include <sys/socket.h>
#include <sys/types.h>

#include <netinet/in.h> // Address struct in here
#include <unistd.h>

int main()
{
    // Create response
    char serverMessage[256] = "You have reached the server!";

    // Create server socket
    int serverSocket = socket(AF_INET, SOCK_STREAM, 0);

    // define address
    struct sockaddr_in serverAddress;
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(8000);
    serverAddress.sin_addr.s_addr = INADDR_ANY;

    // Bind address to socket
    bind(serverSocket, (struct sockaddr *)&serverAddress, sizeof(serverAddress));

    int connectionStatus = listen(serverSocket, 5);

    if (connectionStatus == -1)
    {
        printf("Error occurred connecting to socket\n\n");
    }

    int clientSocket = accept(serverSocket, NULL, NULL);

    // send the message
    send(clientSocket, serverMessage, sizeof(serverMessage), 0);

    // close the socket
    close(serverSocket);

    return 0;
}