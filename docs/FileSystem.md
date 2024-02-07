# File System

## Core Features
- B+ Tree Directory Structure
- Extend-Based Allocation & Chunking with Deduplication
- Full-Disk Encryption
- Snapshotting and Versioning
- Integrated Search and Indexing


## B+ Tree Directory Structure
### Structure
- Hierarchical structure of directories and files in a B+ tree structure.
- B+ trees offer fast search, insertion and deletion operations.
- Each child node is a directory, and each leaf node is a file.
- There is always 1 dummy file to ensure folders are never leaf nodes.

### Moving Files or Entire Directories
- This is a `O(1)` operation.
- Remove the pointer to the file or directory from the old location.
- Add the pointer to the new location.

### Copying files
- This is a `O(1)` operation.
- Because of chunking with deduplication, the file is not actually copied.
- The pointer to the new file is added in the target directory.
- The file is made out of already existing chunks; increment the reference count for each chunk.

### File deletion
- This is a `O(1)` operation.
- Remove the pointer to the file from the directory.
- Check if the reference count for each chunk is 0, and remove the chunk if it is.

## Extend-Based Allocation & Chunking with Deduplication
### Overview
- Extent-based allocation is used to allocate space for files.
- Break files into fixed-size chunks, and store them in a chunk store.
- Deduplicate chunks to save space, and reduce storage costs.

### Store a chunk
- Create chunk identifiers by hashing the chunk data, and use them as keys in the chunk store.
- If the chunk identifier is not found in the chunk store, store the chunk data, and return the chunk identifier.
- Otherwise, just return the existing chunk identifier.
- Use a reference count to track the number of references to each chunk and manage their lifecycle.
- When a chunk has no references, remove it from the chunk store.

### Chunk stores
- There is a chunk store per user, to ensure separation of data.
- There is a system chunk store for system files.

### Chunk size determination
- Variable sized chunks are used to improve deduplication.
- Use a rolling hash to identify similar chunks.

### Deduplication
- Compare the hash of each chunk with the hashes of existing chunks.
- If a match is found, the chunk is deduplicated, and the reference count is incremented.
- If no match is found, the chunk is stored in the chunk store, and a new chunk identifier is created.

### Reconstruct a file
- Reconstruct a file by retrieving the chunks from the chunk store, and concatenating them in the correct order.

### Security
- Each chunk is individually encrypted, and the chunk store is encrypted at rest.
- Unique keys are derived from the system-wide encryption, a chunk identifier, time, a salt and version number.
- Chunks at rest are re-encrypted periodically by re-salting and re-encrypting the chunk data.

### Performance
- A "hot storage area" is used to store the most frequently accessed chunks, to improve performance.
- Hot storage areas are stored on the fastest available storage in a system.
- Recent and frequently accessed chunks are stored in the hot storage area, and are quickly accessible.

### Metadata
- Store metadata for each chunk, including the chunk identifier, reference count, and salt.
- Versioning information is also stored to track changes to the chunk data.

## Full-Disk Encryption
### Encrypting chunk stores
- The user-specific chunk stores encrypt on login and decrypt on logout.
- The system chunk store is encrypted on shutdown and decrypted on boot.

## Snapshotting and Versioning
### Overview
- Create snapshots of the file system to capture the state of the file system at a specific point in time.
- The entire file isn't saved, only the changes that have been made since the last snapshot.
- Works like GitHub, where each change is a commit, and the current file can be rolled back to any previous state.

### Commit & Push
- Every change made is committed to the snapshot store in the file system.
- Once idle for a short amount of time, pending commits are pushed to the snapshot store.
- The latest snapshot is pulled to the current version saved on the disk.

### Rollback
- Rollback to a previous version of the file system by reverting to a previous snapshot.
- Previews and timestamps can be viewed from the snapshot metadata.

### Branching
- Editing an old snapshot causes a branch in the file system, and a new snapshot is created.
- It doesn't remove the other snapshots following the old snapshot.
- The tree system and associated metadata can be viewed in a tree-like structure.

## Integrated Search
### Searching
- The entire filesystem can be searched quickly because of the B+ tree structure.
- The search time is `O(log n)`.
- Chunk metadata includes file size, so folder size analysis is quick.