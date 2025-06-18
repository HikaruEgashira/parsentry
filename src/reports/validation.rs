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