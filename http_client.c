#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include <sys/types.h>
#include <sys/socket.h>

#include <netinet/in.h>
#include <unistd.h>
#include <arpa/inet.h>

int main(int argc, char *argv[])
{
    char *address;
    address = argv[1];

    int port;

    if (argv[2])
    {
        port = atoi(argv[2]);
    }
    else
    {
        port = 80;
    }

    printf("Connecting to ip: %s\nOn port: %d", address, port);

    int clientSocket = socket(AF_INET, SOCK_STREAM, 0);

    // connect to an address
    struct sockaddr_in remoteAddress = {
        .sin_family = AF_INET,
        .sin_port = htons(port),
    };
    inet_aton(address, (struct in_addr *)&remoteAddress.sin_addr.s_addr);

    int connectionStatus = connect(clientSocket, (struct sockaddr *)&remoteAddress, sizeof(remoteAddress));

    if (connectionStatus == -1)
    {
        printf("Failed to make connection at %s\n", address);
    }

    char request[] = "GET / HTTP/1.1\r\n\r\n";
    char response[4096];

    send(clientSocket, request, sizeof(request), 0);
    recv(clientSocket, &response, sizeof(response), 0);

    printf("response from the server: \n\n%s\n\n", response);

    close(clientSocket);

    return 0;
}