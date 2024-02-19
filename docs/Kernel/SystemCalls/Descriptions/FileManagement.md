# File Management
## `FileCreate`
### Description
- Creates either a file or directory inside the `dir` directory.
- A file is created if the `dir` has an extension, otherwise a directory is created.
- The file or directory is created with default permissions, which can be changed using `FileProtect`.
- A new (empty) file contains 0 chunks to start with.

### Speed
- File addition to tree: `O(1)`

### Errors
- `FileErr::FileAreadyExists`: The file or directory already exists.
- `FileErr::PermissionDenied`: The user does not have permission to create the file or directory.

---

## `FileDelete`
### Description
- Deletes the file or directory inside the `dir` directory.
- Deleting a non-empty directory requires the `force` flag to be set to `true`.
- If a file is deleted, the relevant chunks' reference counts are decremented. 0-chunks are removed.

### Speed
- File deletion from tree: `O(1)`
- Chunk removal: `O(n) => O(1)` (extremely fast operation)

### Errors
- `FileErr::FileNotFound`: The file or directory does not exist.
- `FileErr::PermissionDenied`: The user does not have permission to delete the file or directory.
- `FileErr::DirectoryNotEmpty`: The directory is not empty, and the `force` flag is not set.
- `FileErr::ChunkNotFound`: A chunk was not found.

---

## `FileOpen`
### Description
- Opens the file or directory inside the `dir` directory.
- The flags must be a subset of the file's permissions (default: readonly).
- The file is opened in the mode specified by the flags.
- Files adhere to the law of exclusivity: 1 writer or multiple readers.

### Speed
- File opening: `O(1)`

### Errors
- `FileErr::FileNotFound`: The file does not exist.
- `FileErr::PermissionDenied`: The user does not have permission to open the file or directory.
- `FileErr::FileInUse`: The file is already open.
- `FileErr::FileIsDirectory`: The file is a directory.

---

## `FileClose`
### Description
- Closes the file or directory.
- The file is closed, and the file handle is released.
- No operations can be performed on the file after it is closed.

### Speed
- File closing: `O(1)`

### Errors
- `FileErr::FileNotFound`: The file does not exist.
- `FileErr::PermissionDenied`: The user does not have permission to close the file or directory.
- `FileErr::FileNotOpen`: The file is not open.
- `FileErr::FileIsDirectory`: The file is a directory.
- `FileErr::FileInUse`: The file is in use.

---

## `FileWrite`
### Description
- Writes the `data` to the file.
- The file must be opened with the `write` or `append` flag set.
- The data is chunked and deduplicated.
- The chunk ids are stored in the file's metadata.

### Speed
- Chunking: `O(n) => O(1)` (extremely fast operation)

### Errors
- `FileErr::FileNotFound`: The file does not exist.
- `FileErr::PermissionDenied`: The user does not have permission to write to the file.
- `FileErr::FileNotOpen`: The file is not open.
- `FileErr::FileIsDirectory`: The file is a directory.
- `FileErr::ChunkingFailed`: The chunking operation failed.

---

## `FileRead`
### Description
