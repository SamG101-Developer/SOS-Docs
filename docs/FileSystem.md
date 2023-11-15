# File System
## File System Structure
### Hierarchical Structure
- Hierarchical structure of directories and files in a tree-like structure.
- Each directory contains a list of files (leaves) and sub-directories (nodes).
- Each file contains a sequence of bytes, and is identified by a unique path.

### Data Block Allocation
#### Extent-Based Allocation
- Allocate extents (contiguous blocks of data) to each file.
- Reduces fragmentation issues, and improves performance for large files.
- Faster data retrieval, as entire extents can be read in 1 operation.

#### Cluster-Based Allocation
- Allocate data in clusters (groups of sectors on the disk)
- Enable retrieval of multiple sectors in a single operation, to improve performance.
- Reduces disk head movement, and improves data access speed for sequential operations.

### Journaling & Logging
- Implement a VCS (version control system) to track changes to files.
- Allows quick recovery from system crashes, and prevents data loss.
- Allow for a complete file structure reconstruction, if the system crashes.

### Access Control Lists (ACLs)

### Encryption & Authentication
#### Disk Encryption
- Encrypt the entire disk under AES-256, to protect against unauthorized access.


#### Key Management
#### Access Control Mechanisms
#### Multi-Factor Authentication
#### Digital Signatures
#### Secure Tokenization
#### Key Revocation, Expiration & Rotation

