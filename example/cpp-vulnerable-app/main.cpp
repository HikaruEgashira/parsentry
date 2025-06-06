#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <fstream>
#include <sstream>
#include <cstdlib>
#include <cstring>
#include <thread>
#include <mutex>
#include <regex>

class VulnerableDatabase {
private:
    std::vector<std::string> users;
    std::mutex db_mutex;
    
public:
    // SQL Injection vulnerability - direct string concatenation
    std::vector<std::string> searchUsers(const std::string& query) {
        std::vector<std::string> results;
        
        // Vulnerable: Direct SQL construction without parameterization
        std::string sql = "SELECT * FROM users WHERE name = '" + query + "'";
        std::cout << "Executing SQL: " << sql << std::endl;
        
        // Simulate database search
        for (const auto& user : users) {
            if (user.find(query) != std::string::npos) {
                results.push_back(user);
            }
        }
        return results;
    }
    
    void addUser(const std::string& username) {
        // Race condition - no proper locking
        users.push_back(username);
    }
    
    // Memory vulnerability - potential double delete
    std::shared_ptr<std::string> getUserData(int index) {
        if (index >= 0 && index < users.size()) {
            return std::make_shared<std::string>(users[index]);
        }
        return nullptr;
    }
};

class WebServer {
private:
    std::string admin_token;
    VulnerableDatabase* db;
    
public:
    WebServer() : admin_token("admin_secret_123") {
        db = new VulnerableDatabase();
    }
    
    ~WebServer() {
        delete db;
        // Potential double delete if destructor called multiple times
    }
    
    // Command injection vulnerability
    void executeSystemCommand(const std::string& cmd) {
        std::string command = "echo " + cmd;  // No input validation
        std::system(command.c_str());  // Dangerous system call
    }
    
    // Path traversal vulnerability
    std::string readFile(const std::string& filename) {
        std::ifstream file(filename);  // No path validation
        std::stringstream buffer;
        
        if (file.is_open()) {
            buffer << file.rdbuf();
            file.close();
            return buffer.str();
        }
        return "File not found";
    }
    
    // Buffer overflow via C-style operations
    void processUserInput(const char* input) {
        char buffer[256];
        strcpy(buffer, input);  // Unsafe C-style copy
        std::cout << "Processed: " << buffer << std::endl;
    }
    
    // Format string vulnerability
    void logMessage(const std::string& message) {
        printf(message.c_str());  // Dangerous format string usage
        printf("\n");
    }
    
    // Authentication bypass vulnerability
    bool authenticate(const std::string& token) {
        // Timing attack vulnerability - string comparison timing
        return token == admin_token;
    }
    
    // Integer overflow vulnerability
    void allocateBuffer(size_t size) {
        if (size > 0) {
            size_t total_size = size * sizeof(char) * 2;  // Potential overflow
            char* buffer = new char[total_size];
            std::cout << "Allocated " << total_size << " bytes" << std::endl;
            delete[] buffer;
        }
    }
    
    VulnerableDatabase* getDatabase() {
        return db;
    }
};

// Global vulnerable functions
std::string global_config;
std::mutex config_mutex;

void updateGlobalConfig(const std::string& config) {
    // Race condition - improper locking
    global_config = config;
}

std::string getGlobalConfig() {
    // Race condition - reading without lock
    return global_config;
}

// Template vulnerability - type confusion
template<typename T>
void processData(T data) {
    // Unsafe casting without type checking
    char* ptr = reinterpret_cast<char*>(&data);
    std::cout << "Processing: " << ptr << std::endl;
}

// RAII violation - resource leak
class LeakyResource {
private:
    FILE* file_handle;
    
public:
    LeakyResource(const std::string& filename) {
        file_handle = fopen(filename.c_str(), "r");
    }
    
    // Missing destructor - resource leak
    // ~LeakyResource() { if (file_handle) fclose(file_handle); }
    
    void process() {
        if (file_handle) {
            char buffer[1024];
            while (fgets(buffer, sizeof(buffer), file_handle)) {
                std::cout << buffer;
            }
        }
    }
};

// Use after free vulnerability
class VulnerableContainer {
private:
    std::vector<std::unique_ptr<std::string>> items;
    
public:
    void addItem(const std::string& item) {
        items.push_back(std::make_unique<std::string>(item));
    }
    
    void clearItems() {
        items.clear();
    }
    
    // Dangerous - accessing after clear
    void printFirstItem() {
        if (!items.empty()) {
            std::cout << *items[0] << std::endl;  // Use after free
        }
    }
};

int main(int argc, char* argv[]) {
    std::cout << "=== Vulnerable C++ Application ===" << std::endl;
    std::cout << "Warning: This application contains intentional security vulnerabilities!" << std::endl << std::endl;
    
    WebServer server;
    VulnerableContainer container;
    
    // Command line argument vulnerabilities
    if (argc > 1) {
        std::cout << "Processing argument: ";
        printf(argv[1]);  // Format string vulnerability
        std::cout << std::endl;
        
        // Buffer overflow from command line
        if (strlen(argv[1]) > 100) {
            server.processUserInput(argv[1]);
        }
    }
    
    // Environment variable access without validation
    char* debug_env = std::getenv("DEBUG");
    if (debug_env) {
        std::cout << "Debug mode: " << debug_env << std::endl;
        server.logMessage(std::string(debug_env));
    }
    
    // Interactive input handling
    std::string user_input;
    std::cout << "Enter a command: ";
    if (std::getline(std::cin, user_input)) {
        // Multiple vulnerability patterns
        server.logMessage(user_input);  // Format string
        
        // Check for special commands
        if (user_input.substr(0, 5) == "exec:") {
            server.executeSystemCommand(user_input.substr(5));  // Command injection
        }
        
        if (user_input.substr(0, 5) == "file:") {
            std::string content = server.readFile(user_input.substr(5));  // Path traversal
            std::cout << "File content: " << content << std::endl;
        }
        
        if (user_input.substr(0, 6) == "alloc:") {
            size_t size = std::stoull(user_input.substr(6));
            server.allocateBuffer(size);  // Integer overflow
        }
        
        if (user_input.substr(0, 3) == "db:") {
            auto results = server.getDatabase()->searchUsers(user_input.substr(3));  // SQL injection
            for (const auto& result : results) {
                std::cout << "Found user: " << result << std::endl;
            }
        }
    }
    
    // Demonstrate various vulnerabilities
    
    // Use after free
    container.addItem("test item");
    container.clearItems();
    container.printFirstItem();  // Use after free
    
    // Resource leak
    LeakyResource resource("config.txt");
    resource.process();
    // Resource not properly cleaned up
    
    // Race condition
    std::thread t1([&]() {
        updateGlobalConfig("config1");
    });
    
    std::thread t2([&]() {
        updateGlobalConfig("config2");
    });
    
    t1.join();
    t2.join();
    
    std::cout << "Final config: " << getGlobalConfig() << std::endl;
    
    // Type confusion
    int dangerous_data = 0x41414141;
    processData(dangerous_data);
    
    // Authentication test
    std::cout << "Testing authentication..." << std::endl;
    if (server.authenticate("wrong_token")) {
        std::cout << "Authentication successful" << std::endl;
    } else {
        std::cout << "Authentication failed" << std::endl;
    }
    
    std::cout << "Application completed." << std::endl;
    return 0;
}