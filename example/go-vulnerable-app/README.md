# Go Vulnerable Application

This is an intentionally vulnerable Go web application for testing security analysis tools. DO NOT USE IN PRODUCTION.

## Overview

This application contains intentional security vulnerabilities for testing the vulnhuntrs tool. The specific vulnerabilities and their details can be found in the root `answer.md` file.

The application implements several endpoints that demonstrate common web application security issues:

- `/sqli` - SQL query endpoint
- `/xss` - Template rendering endpoint
- `/cmdi` - System command endpoint
- `/file` - File access endpoint

## Setup

```bash
# Initialize dependencies
go mod tidy

# Run the application
go run main.go
```

The server will start at `http://127.0.0.1:8080`

## Security Notice

This application contains intentional security vulnerabilities for testing purposes. DO NOT deploy in production or expose to public networks.