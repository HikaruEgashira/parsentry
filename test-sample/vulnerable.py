#!/usr/bin/env python3

def unsafe_query(user_id):
    """Vulnerable SQL injection example"""
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return execute_query(query)

def execute_query(sql):
    """Mock database execution"""
    print(f"Executing: {sql}")
    return []

# Test vulnerable function
if __name__ == "__main__":
    user_input = input("Enter user ID: ")
    result = unsafe_query(user_input)
    print(result)