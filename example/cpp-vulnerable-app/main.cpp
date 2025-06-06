#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <memory>
#include <cstring>
#include <cstdlib>

class VulnerableClass {
private:
    char* buffer;
    int size;

public:
    // Constructor with potential vulnerability
    VulnerableClass(int s) : size(s) {
        buffer = new char[size];  // Raw pointer usage
    }
    
    // Missing virtual destructor - potential issue in inheritance
    ~VulnerableClass() {
        delete[] buffer;
    }
    
    // Buffer overflow vulnerability
    void copyData(const std::string& input) {
        strcpy(buffer, input.c_str());  // Unsafe copy
    }
    
    // Use after free vulnerability
    void freeBuffer() {
        delete[] buffer;
        buffer = nullptr;
    }
    
    void useBuffer() {
        if (buffer) {
            std::cout << "Buffer: " << buffer << std::endl;
        }
    }
};

// SQL injection-like vulnerability (simulated)
class DatabaseQuery {
public:
    static void executeQuery(const std::string& userInput) {
        std::string query = "SELECT * FROM users WHERE name = '" + userInput + "'";
        // Vulnerable: Direct string concatenation
        std::cout << "Executing: " << query << std::endl;
    }
};

// Command injection vulnerability
void executeCommand(const std::string& userCommand) {
    std::string command = "ls " + userCommand;
    system(command.c_str());  // Dangerous: command injection
}

// Format string vulnerability (C-style)
void logMessage(const char* userMessage) {
    printf(userMessage);  // Unsafe format string
    printf("\n");
}

// Path traversal vulnerability
void readFile(const std::string& filename) {
    std::ifstream file(filename);
    if (file.is_open()) {
        std::string line;
        while (getline(file, line)) {
            std::cout << line << std::endl;
        }
        file.close();
    }
}

// Unsafe casting
void unsafeCasting(void* ptr) {
    // Dangerous reinterpret cast
    int* intPtr = reinterpret_cast<int*>(ptr);
    std::cout << "Value: " << *intPtr << std::endl;
}

// Memory leak with smart pointers misuse
std::shared_ptr<int> createLeakyPointer() {
    int* rawPtr = new int(42);
    // Creating shared_ptr from raw pointer after manual allocation
    return std::shared_ptr<int>(rawPtr);
}

// Double free vulnerability
void doubleFreeVuln() {
    char* ptr = new char[100];
    strcpy(ptr, "Hello");
    
    delete[] ptr;
    // Double free - undefined behavior
    delete[] ptr;
}

// Integer overflow
int calculateArraySize(int elements, int elementSize) {
    return elements * elementSize;  // Can overflow
}

// Unsafe array access
void accessArray(std::vector<int>& vec, int index) {
    // No bounds checking
    std::cout << "Element: " << vec[index] << std::endl;
}

// Template with potential issues
template<typename T>
class UnsafeContainer {
private:
    T* data;
    size_t capacity;
    
public:
    UnsafeContainer(size_t cap) : capacity(cap) {
        data = new T[capacity];
    }
    
    ~UnsafeContainer() {
        delete[] data;  // Should be delete[] for arrays
    }
    
    T& get(size_t index) {
        return data[index];  // No bounds checking
    }
};

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Usage: " << argv[0] << " <input>" << std::endl;
        return 1;
    }
    
    std::string userInput = argv[1];
    
    // Trigger various vulnerabilities
    VulnerableClass obj(64);
    obj.copyData(userInput);
    
    DatabaseQuery::executeQuery(userInput);
    executeCommand(userInput);
    logMessage(userInput.c_str());
    readFile(userInput);
    
    // Memory management issues
    doubleFreeVuln();
    auto leakyPtr = createLeakyPointer();
    
    // Unsafe operations
    char buffer[100];
    unsafeCasting(buffer);
    
    std::vector<int> vec = {1, 2, 3, 4, 5};
    accessArray(vec, 10);  // Out of bounds
    
    // Template usage
    UnsafeContainer<int> container(10);
    container.get(15);  // Out of bounds
    
    return 0;
}