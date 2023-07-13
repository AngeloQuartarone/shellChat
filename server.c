#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <pthread.h>

#define BUF_SIZE 1024
#define IP "192.168.1.22"
#define PORT 8080

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

void *fun_sender(void *arg);
void *fun_receiver(void *arg);
void trim_newline(char *str);

typedef struct
{
    int clientfd;
    char *name;
} str_for_thread_sender;

typedef struct
{
    int clientfd;
    char *name;
} str_for_thread_receiver;

int main(int argc, char *argv[])
{
    // Create a socket.
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0)
    {
        perror("socket");
        exit(1);
    }

    setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &(int){1}, sizeof(int));

    // Bind the socket to a port.
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(PORT);
    addr.sin_addr.s_addr = inet_addr(IP);
    if (bind(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0)
    {
        perror("bind");
        exit(1);
    }

    // Listen for connections.
    if (listen(sockfd, 1) < 0)
    {
        perror("listen");
        exit(1);
    }

    // Accept a connection.
    int clientfd = accept(sockfd, NULL, NULL);
    if (clientfd < 0)
    {
        perror("accept");
        exit(1);
    }

    char *myName = calloc(1, BUF_SIZE);
    char *otherName = calloc(1, BUF_SIZE);
    dprintf(2, "Enter your name: ");
    fflush(stdout);
    scanf("%s", myName);
    fflush(stdin);
    str_for_thread_sender str_sender;
    str_sender.name = malloc(BUF_SIZE);
    str_for_thread_receiver str_receiver;
    str_receiver.name = malloc(BUF_SIZE);
    str_sender.clientfd = clientfd;
    strcpy(str_sender.name, myName);
    send(clientfd, myName, strlen(myName), 0);
    recv(clientfd, otherName, BUF_SIZE, 0);
    trim_newline(otherName);
    str_receiver.clientfd = clientfd;
    strcpy(str_receiver.name, otherName);

    pthread_t sender, receiver;
    pthread_create(&sender, NULL, fun_sender, &str_sender);
    pthread_create(&receiver, NULL, fun_receiver, &str_receiver);
    pthread_detach(sender);
    pthread_detach(receiver);

    while (1)
    {
        sleep(1);
    }

    return 0;
}

void *fun_sender(void *arg)
{
    int clientfd = ((str_for_thread_sender *)arg)->clientfd;
    char *name = ((str_for_thread_sender *)arg)->name;
    //trim_newline(name);

    char message[1024];
    while (1)
    {
        pthread_mutex_lock(&mutex);
        //dprintf(2, "%s > ", name);
        pthread_mutex_unlock(&mutex);
        fgets(message, 1024, stdin);
        if(strcmp(message, "\n") == 0)
        {
            continue;
        }
        send(clientfd, message, strlen(message), 0);
    }
}

void *fun_receiver(void *arg)
{
    int clientfd = ((str_for_thread_receiver *)arg)->clientfd;
    char *name = ((str_for_thread_receiver *)arg)->name;
    char message[1024];
    while (1)
    {
        if (recv(clientfd, message, 1024, 0) == 0)
        {
            break;
        }
        pthread_mutex_lock(&mutex);
        dprintf(2, "\n%s > %s\n", name, message);
        fflush(stdout);
        pthread_mutex_unlock(&mutex);
    }
    return NULL;
}

void trim_newline(char *str)
{
    int len = strlen(str);
    if (str[len - 1] == '\n')
    {
        str[len - 1] = '\0';
    }
}
