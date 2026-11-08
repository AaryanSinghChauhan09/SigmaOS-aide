//! File integrity monitoring file definitions.

/// File attributes
#[derive(Debug, Clone)]
pub struct FileAttributes {
    /// File permissions (octal)
    pub permissions: String,
    /// File owner
    pub owner: String,
    /// File group
    pub group: String,
    /// File size in bytes
    pub size: u64,
    /// Modification time
    pub mtime: u64,
    /// Inode number
    pub inode: u64,
    /// Device number
    pub device: u64,
    /// Number of hard links
    pub links: u32,
}

/// File hash
#[derive(Debug, Clone)]
pub struct FileHash {
    /// MD5 hash
    pub md5: String,
    /// SHA1 hash
    pub sha1: String,
    /// SHA256 hash
    pub sha256: String,
    /// SHA512 hash
    pub sha512: String,
}

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File path
    pub path: String,
    /// File attributes
    pub attributes: FileAttributes,
    /// File hash
    pub hash: FileHash,
    /// File type (file, directory, symlink, etc.)
    pub file_type: FileType,
    /// Is executable
    pub executable: bool,
    /// Is writable
    pub writable: bool,
    /// Is readable
    pub readable: bool,
}

/// File type
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Block device
    BlockDevice,
    /// Character device
    CharacterDevice,
    /// FIFO
    Fifo,
    /// Socket
    Socket,
    /// Unknown
    Unknown,
}

impl FileEntry {
    /// Create new file entry
    pub fn new(path: String, attributes: FileAttributes, hash: FileHash, file_type: FileType) -> Self {
        let executable = attributes.permissions.contains('7') || attributes.permissions.contains('5');
        let writable = attributes.permissions.contains('6') || attributes.permissions.contains('4') || attributes.permissions.contains('2');
        let readable = attributes.permissions.contains('4') || attributes.permissions.contains('2') || attributes.permissions.contains('1');
        
        Self {
            path,
            attributes,
            hash,
            file_type,
            executable,
            writable,
            readable,
        }
    }
    
    /// Get file extension
    pub fn get_extension(&self) -> Option<&str> {
        self.path.rsplit('.').next()
    }
    
    /// Check if file is executable
    pub fn is_executable(&self) -> bool {
        self.executable
    }
    
    /// Check if file is writable
    pub fn is_writable(&self) -> bool {
        self.writable
    }
    
    /// Check if file is readable
    pub fn is_readable(&self) -> bool {
        self.readable
    }
    
    /// Get file size in human readable format
    pub fn get_size_human(&self) -> String {
        let size = self.attributes.size;
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
    
    /// Calculate file age in days
    pub fn get_age_days(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now - self.attributes.mtime) / 86400
    }
    
    /// Check if file is older than specified days
    pub fn is_older_than(&self, days: u64) -> bool {
        self.get_age_days() > days
    }
    
    /// Get file checksum (SHA256)
    pub fn get_checksum(&self) -> &str {
        &self.hash.sha256
    }
    
    /// Verify file hash
    pub fn verify_hash(&self, expected_hash: &str) -> bool {
        self.hash.sha256 == expected_hash || 
        self.hash.md5 == expected_hash || 
        self.hash.sha1 == expected_hash || 
        self.hash.sha512 == expected_hash
    }
}
