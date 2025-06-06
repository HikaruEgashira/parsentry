# C Vulnerable Application

This is a deliberately vulnerable C application designed for security testing and analysis with the vulnhuntrs scanner.

## Vulnerabilities Included

### Memory Safety Vulnerabilities
1. **Buffer Overflow** - `strcpy()` without bounds checking
2. **Use After Free** - Accessing freed memory
3. **Double Free** - Potential for freeing already freed memory
4. **Memory Leaks** - Unreleased allocated memory

### Input Validation Vulnerabilities  
1. **Format String** - `printf()` with user-controlled format strings
2. **Command Injection** - Unsafe `system()` calls with user input
3. **Path Traversal** - File operations without path validation
4. **Integer Overflow** - Unchecked arithmetic in memory allocation

### Concurrency Vulnerabilities
1. **Race Conditions** - Shared variable access without synchronization
2. **TOCTOU** - Time-of-check-time-of-use issues

### Cryptographic Vulnerabilities
1. **Timing Attacks** - String comparison timing side channels
2. **Weak Authentication** - Simple string comparison for passwords

## Building and Running

```bash
# Build the application
make

# Run the application
make run

# Clean build artifacts
make clean
```

## Testing with vulnhuntrs

```bash
# Analyze the C application
cargo run -- -r example/c-vulnerable-app

# Generate detailed reports
cargo run -- -r example/c-vulnerable-app --output-dir ./reports --summary
```

## Warning

⚠️ **This application contains severe security vulnerabilities by design.**

- Do not use in production environments
- Only run in isolated testing environments
- Do not expose to networks or untrusted input
- Use only for security research and educational purposes

## Example Exploits

### Buffer Overflow
```bash
# Input a long string to trigger buffer overflow
echo "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" | ./vulnerable_app
```

### Format String Attack
```bash
# Use format specifiers to read memory
echo "%x %x %x %x" | ./vulnerable_app
```

### Command Injection
```bash
# Execute arbitrary commands
echo "exec:; cat /etc/passwd" | ./vulnerable_app
```

### Path Traversal
```bash
# Access files outside intended directory
echo "file:../../../etc/passwd" | ./vulnerable_app
```