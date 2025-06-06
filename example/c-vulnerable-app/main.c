#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// Buffer overflow vulnerability
void vulnerable_function(char *input) {
    char buffer[64];
    strcpy(buffer, input);  // Unsafe: no bounds checking
    printf("Buffer contains: %s\n", buffer);
}

// Command injection vulnerability
void execute_command(char *user_input) {
    char command[256];
    sprintf(command, "echo %s", user_input);  // Unsafe: command injection
    system(command);  // Dangerous: arbitrary command execution
}

// Integer overflow vulnerability
int calculate_size(int count, int item_size) {
    return count * item_size;  // Can overflow
}

// Memory leak vulnerability
char* allocate_memory(int size) {
    char *ptr = malloc(size);
    if (ptr == NULL) {
        printf("Memory allocation failed\n");
        return NULL;
    }
    // Memory never freed - leak
    return ptr;
}

// Format string vulnerability
void log_message(char *user_message) {
    printf(user_message);  // Unsafe: format string vulnerability
    printf("\n");
}

// Use after free vulnerability
void use_after_free_vuln() {
    char *ptr = malloc(100);
    strcpy(ptr, "Hello World");
    
    free(ptr);
    
    // Use after free - undefined behavior
    printf("String: %s\n", ptr);
}

// Path traversal vulnerability
void read_file(char *filename) {
    FILE *file;
    char buffer[1024];
    
    // No path validation - directory traversal
    file = fopen(filename, "r");
    if (file) {
        fread(buffer, 1, sizeof(buffer), file);
        printf("File content: %s\n", buffer);
        fclose(file);
    }
}

// Unsafe gets function
void read_user_input() {
    char input[100];
    printf("Enter input: ");
    gets(input);  // Deprecated and unsafe
    printf("You entered: %s\n", input);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <input>\n", argv[0]);
        return 1;
    }
    
    // Multiple vulnerabilities triggered
    vulnerable_function(argv[1]);
    execute_command(argv[1]);
    
    char *ptr = allocate_memory(1024);
    log_message(argv[1]);
    
    read_file(argv[1]);
    read_user_input();
    use_after_free_vuln();
    
    int size = calculate_size(1000000, 1000000);
    printf("Calculated size: %d\n", size);
    
    return 0;
}