#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/socket.h>
#include <sys/types.h>

#include <netinet/in.h> // Address struct in here
#include <unistd.h>

int main()
{
    const int port = 8000;

    // open a file to server
    FILE *htmlData = fopen("index.html", "r");

    char responseData[1024];

    fgets(responseData, 1024, htmlData);

    char httpHeader[2048] = "HTTP/1.1 200 OK\r\n\n";

    strcat(httpHeader, responseData);

    // create a socket
    int serverSocket = socket(AF_INET, SOCK_STREAM, 0);

    // define address
    struct sockaddr_in serverAddress;
    serverAddress.sin_port = htons(port);
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_addr.s_addr = INADDR_ANY;

    bind(serverSocket, (struct sockaddr *)&serverAddress, sizeof(serverAddress));

    listen(serverSocket, 5);

    printf("Listening on port %d\n\n", port);

    int clientSocket;

    while (1)
    {
        clientSocket = accept(serverSocket, NULL, NULL);

        printf("Request has been made\n");

        send(clientSocket, httpHeader, sizeof(httpHeader), 0);

        close(clientSocket);
    }

    return 0;
}