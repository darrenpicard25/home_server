#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8000
#define MAXLINE 500

char *bin2Hex(const unsigned char *input, size_t len)
{
    char *results;
    char *hexits = "0123456789ABCDEF";

    if (input == NULL || len <= 0)
    {
        return NULL;
    }

    int resultLength = (len * 3) + 1;

    results = malloc(resultLength);
    bzero(results, resultLength);

    for (int i = 0; i < len; i++)
    {
        results[i * 3] = hexits[input[i] >> 4];
        results[(i * 3) + 1] = hexits[input[i] & 0x0F];
        results[(i * 3) + 2] = ' ';
    }

    return results;
}

void instantiateSocket(int *socketDescriptor, struct sockaddr_in *address)
{
    // Create Socket
    if ((*socketDescriptor = socket(AF_INET, SOCK_STREAM, 0)) == -1)
    {
        perror("An error occurred creating socket");
        exit(1);
    }
    // Zero out
    bzero(address, sizeof(*address));
    // Set Up Address
    address->sin_family = AF_INET;
    address->sin_port = htons(PORT);
    address->sin_addr.s_addr = htonl(INADDR_ANY);

    // Bind Socket to Address
    if (bind(*socketDescriptor, (struct sockaddr *)address, sizeof(*address)) == -1)
    {
        perror("An error occurred binding address to socket");
        exit(1);
    }

    // Start listening on port
    if (listen(*socketDescriptor, 1) == -1)
    {
        perror("An error occurred while listening");
        exit(1);
    }
}

int main()
{
    int socketDescriptor, newSocket;
    struct sockaddr_in address, clientAddr;
    socklen_t clientAddrLength;
    char client_address[MAXLINE];
    char buffer[MAXLINE];

    instantiateSocket(&socketDescriptor, &address);

    for (;;)
    {
        printf("Waiting on connection on port %d\n", PORT);
        if ((newSocket = accept(socketDescriptor, (struct sockaddr *)&clientAddr, &clientAddrLength)) == -1)
        {
            perror("An error occurred accepting a request");
            exit(1);
        }

        inet_ntop(AF_INET, &clientAddr, client_address, MAXLINE);

        if (strncmp(client_address, "2.0.208.174", 7) == 0)
        {
            printf("Success\n");
        }
        printf("Client connection: %s\n", client_address);

        ssize_t n;
        memset(buffer, 0, MAXLINE);
        while ((n = read(newSocket, buffer, MAXLINE - 1)) > 0)
        {
            printf("%s\n", buffer);
            printf("%s\n", bin2Hex(buffer, MAXLINE));
            if (strcmp(&buffer[n - 4], "\r\n\r\n") == 0)
            {
                break;
            }
            memset(buffer, 0, MAXLINE);
        }

        write(newSocket, "Hello", sizeof(char) * 5);
        close(newSocket);
    }

    return 0;
}