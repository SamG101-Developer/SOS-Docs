# B+ Tree Directory Structure
## Structure
- Hierarchical structure of directories and files in a B+ tree structure.
- B+ trees offer fast search, insertion and deletion operations.
- Each child node is a directory, and each leaf node is a file.
- There is always 1 dummy file to ensure folders are never leaf nodes.

## Moving Files or Entire Directories
- This is a `O(1)` operation.
- Remove the pointer to the file or directory from the old location.
- Add the pointer to the new location.

## Copying files
- This is a `O(1)` operation.
- Because of chunking with deduplication, the file is not actually copied.
- The pointer to the new file is added in the target directory.
- The file is made of already existing chunks; increment the reference count for each chunk.

## File deletion
- This is a `O(1)` operation.
- Remove the pointer to the file from the directory.
- Check if the reference count for each chunk is 0, and remove the chunk if it is.
