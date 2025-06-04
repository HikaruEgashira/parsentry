package main

import (
	"database/sql"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func main() {
	var err error
	db, err = sql.Open("sqlite3", "users.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	createTable()
	insertSampleData()

	http.HandleFunc("/", indexHandler)
	http.HandleFunc("/sqli", sqliHandler)
	http.HandleFunc("/xss", xssHandler)
	http.HandleFunc("/cmdi", cmdiHandler)
	http.HandleFunc("/file", fileHandler)

	fmt.Println("Server starting at http://127.0.0.1:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func createTable() {
	query := `
	CREATE TABLE IF NOT EXISTS users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT NOT NULL,
		password TEXT NOT NULL
	)`
	_, err := db.Exec(query)
	if err != nil {
		log.Fatal(err)
	}
}

func insertSampleData() {
	users := [][]string{
		{"admin", "admin123"},
		{"user", "password"},
		{"guest", "guest123"},
	}

	for _, user := range users {
		var count int
		err := db.QueryRow("SELECT COUNT(*) FROM users WHERE username = ?", user[0]).Scan(&count)
		if err != nil {
			log.Printf("Error checking user: %v", err)
			continue
		}

		if count == 0 {
			_, err := db.Exec("INSERT INTO users (username, password) VALUES (?, ?)", user[0], user[1])
			if err != nil {
				log.Printf("Error inserting user: %v", err)
			}
		}
	}
}

func indexHandler(w http.ResponseWriter, r *http.Request) {
	html := `
<!DOCTYPE html>
<html>
<head>
    <title>Go Vulnerable Application</title>
</head>
<body>
    <h1>Go Vulnerable Application</h1>
    <p>This is an intentionally vulnerable application for testing security scanners.</p>
    <h2>Endpoints:</h2>
    <ul>
        <li><a href="/sqli?username=admin">SQL Injection Test</a></li>
        <li><a href="/xss?name=test">XSS Test</a></li>
        <li><a href="/cmdi?hostname=localhost">Command Injection Test</a></li>
        <li><a href="/file?name=README.md">File Access Test</a></li>
    </ul>
</body>
</html>`
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprint(w, html)
}

func sqliHandler(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	if username == "" {
		fmt.Fprint(w, "Usage: /sqli?username=<username>")
		return
	}

	query := fmt.Sprintf("SELECT id, username, password FROM users WHERE username = '%s'", username)
	log.Printf("Executing query: %s", query)

	rows, err := db.Query(query)
	if err != nil {
		http.Error(w, fmt.Sprintf("Database error: %v", err), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "text/html")
	fmt.Fprint(w, "<h2>SQL Injection Results</h2><table border='1'><tr><th>ID</th><th>Username</th><th>Password</th></tr>")

	for rows.Next() {
		var id int
		var user, pass string
		if err := rows.Scan(&id, &user, &pass); err != nil {
			log.Printf("Error scanning row: %v", err)
			continue
		}
		fmt.Fprintf(w, "<tr><td>%d</td><td>%s</td><td>%s</td></tr>", id, user, pass)
	}
	fmt.Fprint(w, "</table>")
}

func xssHandler(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	if name == "" {
		fmt.Fprint(w, "Usage: /xss?name=<name>")
		return
	}

	tmplStr := fmt.Sprintf(`
<!DOCTYPE html>
<html>
<head>
    <title>XSS Test</title>
</head>
<body>
    <h2>Hello, %s!</h2>
    <p>Your input was: %s</p>
</body>
</html>`, name, name)

	tmpl, err := template.New("xss").Parse(tmplStr)
	if err != nil {
		http.Error(w, "Template error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/html")
	tmpl.Execute(w, nil)
}

func cmdiHandler(w http.ResponseWriter, r *http.Request) {
	hostname := r.URL.Query().Get("hostname")
	if hostname == "" {
		fmt.Fprint(w, "Usage: /cmdi?hostname=<hostname>")
		return
	}

	command := fmt.Sprintf("ping -c 1 %s", hostname)
	log.Printf("Executing command: %s", command)

	cmd := exec.Command("sh", "-c", command)
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, fmt.Sprintf("Command error: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	fmt.Fprintf(w, "Command: %s\n\nOutput:\n%s", command, string(output))
}

func fileHandler(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("name")
	if filename == "" {
		fmt.Fprint(w, "Usage: /file?name=<filename>")
		return
	}

	if strings.Contains(filename, "..") {
		log.Printf("Suspicious filename detected: %s", filename)
	}

	content, err := os.ReadFile(filename)
	if err != nil {
		http.Error(w, fmt.Sprintf("File error: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	fmt.Fprintf(w, "File: %s\n\nContent:\n%s", filename, string(content))
}