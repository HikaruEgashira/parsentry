<?php
// Example vulnerable PHP code for testing the scanner

// SQL Injection vulnerability
function getUserData($id) {
    $conn = new mysqli("localhost", "user", "password", "database");
    
    // Direct concatenation of user input into SQL query - vulnerable to SQL injection
    $query = "SELECT * FROM users WHERE id = " . $_GET['id'];
    $result = mysqli_query($conn, $query);
    
    return $result;
}

// Command Injection vulnerability
function processFile($filename) {
    // User input directly passed to system command - vulnerable to command injection
    $output = shell_exec("cat " . $_POST['filename']);
    echo $output;
}

// XSS vulnerability
function displayUserInput() {
    // Direct output of user input without escaping - vulnerable to XSS
    echo "<h1>Welcome " . $_GET['username'] . "</h1>";
}

// File Inclusion vulnerability
function loadTemplate($template) {
    // Direct inclusion of user-specified file - vulnerable to LFI/RFI
    include($_REQUEST['template']);
}

// Code Injection vulnerability
function evaluateFormula($formula) {
    // Direct evaluation of user input - vulnerable to code injection
    eval('$result = ' . $_POST['formula'] . ';');
    return $result;
}

// Insecure Deserialization
function processData($data) {
    // Unserializing user input - vulnerable to object injection
    $obj = unserialize($_COOKIE['data']);
    return $obj;
}

// Path Traversal vulnerability
function readFile($file) {
    // No path validation - vulnerable to directory traversal
    $content = file_get_contents("/var/www/uploads/" . $_GET['file']);
    return $content;
}

// LDAP Injection
function authenticateUser($username, $password) {
    $ldap = ldap_connect("ldap.example.com");
    
    // Direct concatenation in LDAP query - vulnerable to LDAP injection
    $filter = "(&(uid=" . $_POST['username'] . ")(password=" . $_POST['password'] . "))";
    $search = ldap_search($ldap, "dc=example,dc=com", $filter);
    
    return ldap_count_entries($ldap, $search) > 0;
}

// Open Redirect vulnerability
function redirect() {
    // Direct use of user input in redirect - vulnerable to open redirect
    header("Location: " . $_GET['url']);
    exit();
}

// Weak Random Number Generation
function generateToken() {
    // Using predictable random function
    return rand(1000, 9999);
}

// Example of safe code
function safeGetUserData($id) {
    $conn = new mysqli("localhost", "user", "password", "database");
    
    // Using prepared statements - safe from SQL injection
    $stmt = $conn->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->bind_param("i", $id);
    $stmt->execute();
    
    return $stmt->get_result();
}

function safeDisplayUserInput() {
    // Properly escaped output - safe from XSS
    echo "<h1>Welcome " . htmlspecialchars($_GET['username'], ENT_QUOTES, 'UTF-8') . "</h1>";
}