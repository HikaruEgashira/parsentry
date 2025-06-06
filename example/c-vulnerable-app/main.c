#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

#define BUFFER_SIZE 1024
#define MAX_COMMAND_SIZE 256

// Global variables for configuration
char *admin_password = "admin123";
int debug_mode = 0;

// Vulnerable function: Buffer overflow in string copy
void unsafe_string_copy(char *input) {
    char buffer[100];
    strcpy(buffer, input);  // Buffer overflow vulnerability
    printf("Copied: %s\n", buffer);
}

// Vulnerable function: Format string vulnerability
void log_message(char *user_input) {
    printf("Log: ");
    printf(user_input);  // Format string vulnerability
    printf("\n");
}

// Vulnerable function: Command injection
void execute_system_command(char *command) {
    char full_command[MAX_COMMAND_SIZE];
    sprintf(full_command, "echo %s", command);  // Command injection
    system(full_command);  // Dangerous system call
}

// Vulnerable function: Integer overflow
void allocate_memory(unsigned int size) {
    if (size > 0) {
        char *buffer = malloc(size * sizeof(char));  // Integer overflow possible
        if (buffer == NULL) {
            printf("Memory allocation failed\n");
            return;
        }
        printf("Allocated %u bytes\n", size);
        free(buffer);
    }
}

// Vulnerable function: Use after free
char *global_buffer = NULL;

void free_global_buffer() {
    free(global_buffer);
    // Missing: global_buffer = NULL;
}

void use_global_buffer() {
    if (global_buffer != NULL) {
        strcpy(global_buffer, "test");  // Use after free vulnerability
    }
}

// Vulnerable function: Race condition
int shared_counter = 0;

void increment_counter() {
    shared_counter++;  // Race condition without proper locking
}

// Vulnerable authentication function
int authenticate_user(char *username, char *password) {
    // Timing attack vulnerability - strcmp timing can leak information
    if (strcmp(username, "admin") == 0 && strcmp(password, admin_password) == 0) {
        return 1;  // Success
    }
    return 0;  // Failure
}

// Vulnerable network function
void handle_network_input(int socket_fd) {
    char buffer[256];
    int bytes_received;
    
    // No bounds checking on recv
    bytes_received = recv(socket_fd, buffer, 1024, 0);  // Buffer overflow
    if (bytes_received > 0) {
        buffer[bytes_received] = '\0';
        printf("Received: %s\n", buffer);
        
        // Process command without validation
        if (strncmp(buffer, "CMD:", 4) == 0) {
            execute_system_command(buffer + 4);
        }
    }
}

// Vulnerable file operations
void read_config_file(char *filename) {
    FILE *file;
    char buffer[512];
    
    // Path traversal vulnerability - no validation of filename
    file = fopen(filename, "r");
    if (file != NULL) {
        while (fgets(buffer, sizeof(buffer), file) != NULL) {
            printf("Config: %s", buffer);
        }
        fclose(file);
    }
}

// Main function with various vulnerabilities
int main(int argc, char *argv[]) {
    char input_buffer[1024];
    char *user_input;
    
    printf("=== Vulnerable C Application ===\n");
    printf("Warning: This application contains intentional security vulnerabilities!\n\n");
    
    // Command line argument vulnerability
    if (argc > 1) {
        printf("Processing argument: ");
        printf(argv[1]);  // Format string vulnerability
        printf("\n");
        
        // Potential buffer overflow from command line
        if (strlen(argv[1]) > 100) {
            unsafe_string_copy(argv[1]);
        }
    }
    
    // Environment variable access without validation
    char *debug_env = getenv("DEBUG");
    if (debug_env != NULL) {
        debug_mode = atoi(debug_env);  // No validation
        if (debug_mode) {
            printf("Debug mode enabled with value: %s\n", debug_env);
        }
    }
    
    // Simulate interactive input
    printf("Enter a message: ");
    if (fgets(input_buffer, sizeof(input_buffer), stdin) != NULL) {
        input_buffer[strcspn(input_buffer, "\n")] = 0;  // Remove newline
        
        // Multiple vulnerability patterns
        log_message(input_buffer);           // Format string
        unsafe_string_copy(input_buffer);    // Buffer overflow
        
        // Check for special commands
        if (strncmp(input_buffer, "exec:", 5) == 0) {
            execute_system_command(input_buffer + 5);  // Command injection
        }
        
        if (strncmp(input_buffer, "alloc:", 6) == 0) {
            unsigned int size = atoi(input_buffer + 6);
            allocate_memory(size);  // Integer overflow
        }
        
        if (strncmp(input_buffer, "file:", 5) == 0) {
            read_config_file(input_buffer + 5);  // Path traversal
        }
    }
    
    // Memory management vulnerabilities
    global_buffer = malloc(256);
    strcpy(global_buffer, "initial data");
    free_global_buffer();
    use_global_buffer();  // Use after free
    
    // Authentication test
    printf("Testing authentication...\n");
    if (authenticate_user("admin", "wrongpass")) {
        printf("Authentication successful\n");
    } else {
        printf("Authentication failed\n");
    }
    
    printf("Application completed.\n");
    return 0;
}