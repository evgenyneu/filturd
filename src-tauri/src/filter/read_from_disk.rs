use std::path::Path;
use tokio::fs;

pub async fn read_filter_from_disk<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_filter_from_disk() {
        // Create a temporary file with some content
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = "test filter content";
        write!(temp_file, "{}", test_content).unwrap();

        // Read the content using our function
        let result = read_filter_from_disk(temp_file.path()).await;

        // Assert the result matches our test content
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[tokio::test]
    async fn test_read_filter_from_disk_nonexistent_file() {
        // Try to read a file that doesn't exist
        let result = read_filter_from_disk("nonexistent_file.txt").await;

        // Assert that we get an error
        assert!(result.is_err());
    }
}
