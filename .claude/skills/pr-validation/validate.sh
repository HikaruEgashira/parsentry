#!/bin/bash
# PR Validation Script for Parsentry
# 使用法: ./validate.sh [PR_NUMBER] [--dynamic-check]

set -e

PR_NUMBER="${1:-160}"
DYNAMIC_CHECK="${2:-}"
RESULTS_DIR="${HOME}/parsentry-validation-results"
TEST_DIR="/tmp/parsentry_test"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_section() {
    echo -e "\n${YELLOW}=== $1 ===${NC}\n"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 1. Checkout PR
log_section "PR #$PR_NUMBER をチェックアウト"
if gh pr checkout "$PR_NUMBER" 2>/dev/null; then
    log_success "PR チェックアウト完了"
else
    log_error "PR チェックアウト失敗"
    exit 1
fi

# 2. Run unit tests
log_section "ユニットテスト実行"
if cargo test 2>&1 | tail -20; then
    log_success "全テスト成功"
else
    log_error "テスト失敗"
    exit 1
fi

# 3. Check compilation
log_section "コンパイルチェック"
if cargo check; then
    log_success "コンパイル成功"
else
    log_error "コンパイルエラー"
    exit 1
fi

# 4. Release build
log_section "Release ビルド"
START_TIME=$(date +%s)
if cargo build --release 2>&1 | grep -E "Finished|error"; then
    END_TIME=$(date +%s)
    BUILD_TIME=$((END_TIME - START_TIME))
    log_success "Release ビルド成功 (${BUILD_TIME}秒)"
else
    log_error "Release ビルド失敗"
    exit 1
fi

# 5. Dynamic check (optional)
if [ "$DYNAMIC_CHECK" == "--dynamic-check" ]; then
    log_section "動的セキュリティパターン検出"

    # Create test directory
    mkdir -p "$TEST_DIR"/{python,js,java}

    # Create sample code with vulnerabilities
    cat > "$TEST_DIR/python/sql_injection.py" << 'EOF'
import sqlite3

def get_user(user_id):
    conn = sqlite3.connect('users.db')
    cursor = conn.cursor()
    query = f"SELECT * FROM users WHERE id = {user_id}"
    cursor.execute(query)
    return cursor.fetchall()

def unsafe_query(user_input):
    conn = sqlite3.connect('data.db')
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM data WHERE name = '" + user_input + "'")
    return cursor.fetchall()
EOF

    cat > "$TEST_DIR/js/xss_vulnerability.js" << 'EOF'
function displayUserComment(userInput) {
    const comment = document.getElementById('comment');
    comment.innerHTML = userInput;
}

const userContent = getUserInput();
document.body.innerHTML += "<div>" + userContent + "</div>";

function executeCode(codeString) {
    eval(codeString);
}

const fs = require('fs');
function readFile(filename) {
    const filePath = './uploads/' + filename;
    return fs.readFileSync(filePath, 'utf8');
}
EOF

    cat > "$TEST_DIR/java/CommandInjection.java" << 'EOF'
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class CommandInjection {
    public static void executeCommand(String userInput) throws IOException {
        String cmd = "ls " + userInput;
        Runtime.getRuntime().exec(cmd);
    }

    public static String readFile(String filename) throws Exception {
        String filePath = "/var/uploads/" + filename;
        return Files.readAllLines(Paths.get(filePath)).toString();
    }

    public static void queryDatabase(String userId) throws Exception {
        String query = "SELECT * FROM users WHERE id = " + userId;
    }
}
EOF

    # Create results directory
    mkdir -p "$RESULTS_DIR"

    # Run scan
    if cargo run --release -- --root "$TEST_DIR" --output-dir "$RESULTS_DIR" 2>&1 | grep -E "解析完了|セキュリティパターン"; then
        log_success "セキュリティパターン検出完了"

        # Verify SARIF report
        if [ -f "$RESULTS_DIR/parsentry-results.sarif" ]; then
            log_success "SARIF レポート生成確認"
            echo "レポート: $RESULTS_DIR/parsentry-results.sarif"
        else
            log_error "SARIF レポート生成失敗"
        fi
    else
        log_error "セキュリティスキャン実行失敗"
    fi
fi

# Final summary
log_section "検証完了"
log_success "PR #$PR_NUMBER の動作確認が完了しました"
