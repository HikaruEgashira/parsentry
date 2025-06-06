# C Vulnerable Application

This is a deliberately vulnerable C application for testing the vulnerability scanner.

## Vulnerabilities Included

1. **Buffer Overflow** - `strcpy()` without bounds checking
2. **Command Injection** - Unsanitized input passed to `system()`
3. **Format String** - User input passed directly to `printf()`
4. **Memory Leak** - `malloc()` without corresponding `free()`
5. **Use After Free** - Accessing freed memory
6. **Path Traversal** - No validation on file paths
7. **Unsafe Functions** - Use of deprecated `gets()`
8. **Integer Overflow** - Unchecked arithmetic operations

## Building

```bash
make
```

## Running

```bash
./vulnerable_app "test input"
```

**Warning**: This application contains security vulnerabilities and should only be used for testing purposes in a safe environment.