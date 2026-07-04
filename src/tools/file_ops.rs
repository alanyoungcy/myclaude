//! File operation tools for code agents
//!
//! This module provides file system operation tools with security validation.

use rig_core::{
    completion::ToolDefinition,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors for file operations
#[derive(Debug, Error)]
pub enum FileOperationError {
    #[error("File operation failed: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Arguments for reading a file
#[derive(Debug, Deserialize)]
pub struct ReadFileArgs {
    /// Path to the file to read
    pub path: String,
}

/// Read File Tool
///
/// Reads the contents of a text file.
///
/// # Security
/// - Only allows reading files within allowed directories
/// - Rejects absolute paths outside workspace
/// - Validates file exists and is readable
#[derive(Clone, Serialize)]
pub struct ReadFileTool {
    #[serde(skip)]
    allowed_directories: Vec<PathBuf>,
}

impl ReadFileTool {
    /// Create a new read file tool
    ///
    /// # Arguments
    /// * `allowed_directories` - List of allowed base directories for file access
    pub fn new(allowed_directories: Vec<PathBuf>) -> Self {
        Self {
            allowed_directories,
        }
    }

    /// Validate that a path is within allowed directories
    fn validate_path(&self, path: &Path) -> Result<PathBuf, FileOperationError> {
        // Convert to absolute path
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // Canonicalize to resolve symlinks and ..
        let canonical_path = absolute_path
            .canonicalize()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    FileOperationError::FileNotFound(path.display().to_string())
                } else {
                    FileOperationError::IoError(e)
                }
            })?;

        // Check if path is within allowed directories
        let is_allowed = self.allowed_directories.iter().any(|allowed_dir| {
            canonical_path.starts_with(allowed_dir)
        });

        if !is_allowed {
            return Err(FileOperationError::PermissionDenied(format!(
                "Path {} is not in allowed directories",
                canonical_path.display()
            )));
        }

        Ok(canonical_path)
    }
}

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";

    type Error = FileOperationError;
    type Args = ReadFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Read the contents of a text file. Use this to examine source code, configuration files, or any text-based files.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to read (relative or absolute)"
                    }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path
        let validated_path = self.validate_path(path)?;

        // Read file
        let content = tokio::fs::read_to_string(&validated_path).await?;

        Ok(content)
    }
}

/// Arguments for writing a file
#[derive(Debug, Deserialize)]
pub struct WriteFileArgs {
    /// Path to the file to write
    pub path: String,
    /// Content to write to the file
    pub content: String,
}

/// Write File Tool
///
/// Writes content to a text file, creating it if it doesn't exist.
///
/// # Security
/// - Only allows writing files within allowed directories
/// - Creates parent directories if needed
/// - Overwrites existing files
#[derive(Clone, Serialize)]
pub struct WriteFileTool {
    #[serde(skip)]
    allowed_directories: Vec<PathBuf>,
}

impl WriteFileTool {
    /// Create a new write file tool
    pub fn new(allowed_directories: Vec<PathBuf>) -> Self {
        Self {
            allowed_directories,
        }
    }

    /// Validate that a path is within allowed directories
    fn validate_path(&self, path: &Path) -> Result<PathBuf, FileOperationError> {
        // Convert to absolute path
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // Get parent directory and canonicalize it
        let parent = absolute_path
            .parent()
            .ok_or_else(|| FileOperationError::InvalidPath("No parent directory".to_string()))?;

        let canonical_parent = if parent.exists() {
            parent.canonicalize()?
        } else {
            // If parent doesn't exist, we'll create it, but validate the intended parent
            parent.to_path_buf()
        };

        // Check if parent is within allowed directories
        let is_allowed = self.allowed_directories.iter().any(|allowed_dir| {
            canonical_parent.starts_with(allowed_dir)
        });

        if !is_allowed {
            return Err(FileOperationError::PermissionDenied(format!(
                "Path {} is not in allowed directories",
                absolute_path.display()
            )));
        }

        Ok(absolute_path)
    }
}

impl Tool for WriteFileTool {
    const NAME: &'static str = "write_file";

    type Error = FileOperationError;
    type Args = WriteFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Write content to a text file. Creates the file if it doesn't exist, overwrites if it does. Use this to create or update source code, configuration files, etc.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to write (relative or absolute)"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write to the file"
                    }
                },
                "required": ["path", "content"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path
        let validated_path = self.validate_path(path)?;

        // Create parent directories if needed
        if let Some(parent) = validated_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write file
        tokio::fs::write(&validated_path, args.content).await?;

        Ok(format!("Successfully wrote to {}", validated_path.display()))
    }
}

/// Arguments for listing files in a directory
#[derive(Debug, Deserialize)]
pub struct ListFilesArgs {
    /// Path to the directory to list
    pub path: String,
    /// Whether to list recursively (default: false)
    #[serde(default)]
    pub recursive: bool,
}

/// List Files Tool
///
/// Lists files and directories in a given path.
#[derive(Clone, Serialize)]
pub struct ListFilesTool {
    #[serde(skip)]
    allowed_directories: Vec<PathBuf>,
}

impl ListFilesTool {
    /// Create a new list files tool
    pub fn new(allowed_directories: Vec<PathBuf>) -> Self {
        Self {
            allowed_directories,
        }
    }

    /// Validate that a path is within allowed directories
    fn validate_path(&self, path: &Path) -> Result<PathBuf, FileOperationError> {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        let canonical_path = absolute_path.canonicalize()?;

        let is_allowed = self.allowed_directories.iter().any(|allowed_dir| {
            canonical_path.starts_with(allowed_dir)
        });

        if !is_allowed {
            return Err(FileOperationError::PermissionDenied(format!(
                "Path {} is not in allowed directories",
                canonical_path.display()
            )));
        }

        Ok(canonical_path)
    }

    /// List files in a directory (non-recursive)
    async fn list_dir(&self, path: &Path) -> Result<Vec<String>, FileOperationError> {
        let mut entries = Vec::new();
        let mut read_dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<unknown>")
                .to_string();

            let metadata = entry.metadata().await?;
            let entry_type = if metadata.is_dir() { "📁" } else { "📄" };

            entries.push(format!("{} {}", entry_type, file_name));
        }

        entries.sort();
        Ok(entries)
    }
}

impl Tool for ListFilesTool {
    const NAME: &'static str = "list_files";

    type Error = FileOperationError;
    type Args = ListFilesArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List files and directories in a given path. Use this to explore the file system structure.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the directory to list"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "Whether to list recursively (default: false)",
                        "default": false
                    }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path
        let validated_path = self.validate_path(path)?;

        // List files
        let entries = self.list_dir(&validated_path).await?;

        if entries.is_empty() {
            Ok(format!("Directory {} is empty", validated_path.display()))
        } else {
            Ok(format!(
                "Contents of {}:\n{}",
                validated_path.display(),
                entries.join("\n")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_read_file_tool_creation() {
        let tool = ReadFileTool::new(vec![PathBuf::from("/tmp")]);
        assert_eq!(tool.allowed_directories.len(), 1);
    }

    #[test]
    fn test_write_file_tool_creation() {
        let tool = WriteFileTool::new(vec![PathBuf::from("/tmp")]);
        assert_eq!(tool.allowed_directories.len(), 1);
    }

    #[test]
    fn test_list_files_tool_creation() {
        let tool = ListFilesTool::new(vec![PathBuf::from("/tmp")]);
        assert_eq!(tool.allowed_directories.len(), 1);
    }
}
