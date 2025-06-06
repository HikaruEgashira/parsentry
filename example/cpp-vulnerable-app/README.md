# C++ Vulnerable Application

This is a deliberately vulnerable C++ application for testing the vulnerability scanner.

## Vulnerabilities Included

1. **Buffer Overflow** - `strcpy()` in class methods
2. **Command Injection** - Unsanitized input to `system()`
3. **SQL Injection** - Direct string concatenation in queries
4. **Format String** - User input passed to `printf()`
5. **Memory Management Issues**:
   - Use after free
   - Double free
   - Memory leaks with smart pointers
6. **Path Traversal** - No validation on file paths
7. **Unsafe Casting** - `reinterpret_cast` without validation
8. **Integer Overflow** - Unchecked arithmetic
9. **Array Bounds** - No bounds checking in containers
10. **Template Safety** - Unsafe template implementations

## C++-Specific Issues

- Missing virtual destructors
- Raw pointer usage instead of smart pointers
- Unsafe STL container access
- Template instantiation vulnerabilities

## Building

```bash
make
```

## Running

```bash
./vulnerable_cpp_app "test input"
```

**Warning**: This application contains security vulnerabilities and should only be used for testing purposes in a safe environment.