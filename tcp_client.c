#include <stdio.h>
#include <stdlib.h>

#include <sys/socket.h>
#include <sys/types.h>

#include <netinet/in.h> // Address struct in here
#include <unistd.h>

int main()
{
    // Create socket
    int networkSocket = socket(AF_INET, SOCK_STREAM, 0);

    // Set up address for socket
    struct sockaddr_in serverAddress;
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(8000);
    serverAddress.sin_addr.s_addr = INADDR_ANY;

    // connect socket to address
    int connectionStatus = connect(networkSocket, (struct sockaddr *)&serverAddress, sizeof(serverAddress));

    if (connectionStatus != 0)
    {
        printf("There was an error making connection to the remote socket \n\n");
    }

    // Receive data from server
    char serverResponse[256];

    recv(networkSocket, &serverResponse, sizeof(serverResponse), 0);

    // Print out data received from server
    printf("The server sent data: %s", serverResponse);

    // Close the socket
    close(networkSocket);

    return 0;
}