#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    #define BUF_SIZE 1024
    char buffer[BUF_SIZE];
    size_t contentSize = 1; // includes NULL
    /* Preallocate space.  We could just allocate one char here, 
    but that wouldn't be efficient. */
    char * content = malloc(sizeof(char) * BUF_SIZE);
    if (content == NULL) {
        perror("Failed to allocate content");
        exit(1);
    }
    content[0] = '\0'; // make null-terminated
    while (fgets(buffer, BUF_SIZE, stdin)) {
        char * old = content;
        contentSize += strlen(buffer);
        content = realloc(content, contentSize);
        if (content == NULL) {
            perror("Failed to reallocate content");
            free(old);
            exit(2);
        }
        strcat(content, buffer);
    }

    if (ferror(stdin)) {
        free(content);
        perror("Error reading from stdin.");
        exit(3);
    }

    printf(content);
}
