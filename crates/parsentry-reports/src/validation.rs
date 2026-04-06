use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn validate_output_directory(output_dir: &PathBuf) -> Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|e| anyhow::anyhow!("ディレクトリの作成に失敗: {}", e))?;
    }

    let mut test_file_path = output_dir.clone();
    test_file_path.push(".parsentry_write_test");

    match fs::File::create(&test_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(b"test") {
                let _ = fs::remove_file(&test_file_path);
                return Err(anyhow::anyhow!("書き込み権限がありません: {}", e));
            }
            drop(file);
            fs::remove_file(&test_file_path)
                .map_err(|e| anyhow::anyhow!("テストファイルの削除に失敗: {}", e))?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("ファイル作成権限がありません: {}", e));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_validate_existing_writable_directory() {
        let dir = tempdir().unwrap();
        let result = validate_output_directory(&dir.path().to_path_buf());
        assert!(result.is_ok());
        // The test file should be cleaned up
        assert!(!dir.path().join(".parsentry_write_test").exists());
    }

    #[test]
    fn test_validate_creates_nonexistent_directory() {
        let dir = tempdir().unwrap();
        let new_dir = dir.path().join("subdir").join("nested");
        assert!(!new_dir.exists());

        let result = validate_output_directory(&new_dir);
        assert!(result.is_ok());
        assert!(new_dir.exists());
    }

    #[test]
    fn test_validate_deeply_nested_directory() {
        let dir = tempdir().unwrap();
        let deep = dir.path().join("a").join("b").join("c").join("d");
        assert!(!deep.exists());

        let result = validate_output_directory(&deep);
        assert!(result.is_ok());
        assert!(deep.exists());
    }

    #[test]
    fn test_validate_already_existing_directory_idempotent() {
        let dir = tempdir().unwrap();
        let result1 = validate_output_directory(&dir.path().to_path_buf());
        assert!(result1.is_ok());
        let result2 = validate_output_directory(&dir.path().to_path_buf());
        assert!(result2.is_ok());
    }
}
