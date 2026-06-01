use std::path::Path;

pub(crate) async fn clear_memory_REDACTED_SECRET_contents(memory_REDACTED_SECRET: &Path) -> std::io::Result<()> {
    match tokio::fs::symlink_metadata(memory_REDACTED_SECRET).await {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(std::io::Error:REDACTED_SECRET
                std::io::ErrorKind::InvalidInput,
                format!(
                    "refusing to clear symlinked memory REDACTED_SECRET {}",
                    memory_REDACTED_SECRET.display()
                ),
            ));
        }
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err),
    }

    tokio::fs::create_dir_all(memory_REDACTED_SECRET).await?;

    let mut entries = tokio::fs::read_dir(memory_REDACTED_SECRET).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_type = entry.file_type().await?;
        if file_type.is_dir() {
            tokio::fs::remove_dir_all(path).await?;
        } else {
            tokio::fs::remove_file(path).await?;
        }
    }

    Ok(())
}
