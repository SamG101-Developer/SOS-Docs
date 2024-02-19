# B+ Tree Directory Structure
## Structure
- Hierarchical structure of directories and files in a B+ tree structure.
- B+ trees offer fast search, insertion and deletion operations.
- Each child node is a directory, and each leaf node is a file.
- There is always 1 dummy file to ensure folders are never leaf nodes.

## Moving Files / Directories
- This is a `O(1)` operation.
- Remove the pointer to the file or directory from the old location.
- Add the pointer to the new location.

## Copying Files / Directories
- This is a `O(1)` operation.
- Because of chunking with deduplication, the file is not actually copied.
- The pointer to the new file is added in the target directory.
- The file is made of already existing chunks; increment the reference count for each chunk.

## Deleting Files / Directories
- This is a `O(1)` operation.
- Remove the pointer to the file from the directory.
- Check if the reference count for each chunk is 0, and remove the chunk if it is.

## Directory structure
- Like Windows, the drive is the root directory, ie `C:/`.
- Under the root directory are the users, including the `System` user.
- This folder cannot be modified by the user.
- The path separation character is always the `/` character (url compatible).

### Users & Profiles
- Each user has a folder in the root directory.
- Each user can have multiple profiles, with a sub-folder per profile.
- Each profile can individually have its own applications, settings and data.

### User's folders
- `Videos`: Contains the videos.
- `Documents`: Contains the documents.
- `Pictures`: Contains the pictures.
- `Music`: Contains the music.
- `Downloads`: Contains the downloads.
- `Desktop`: Contains the desktop files.
- `Applications`: Contains user-specific applications (sub-folder per application).
- `Trash`: Contains the deleted files.
- `Temp`: Contains the temporary files.
- No other folders can be added or removed in the user root directory.
- The `System` user doesn't have some of these folders.

### Folder names
- Folder names must start with an uppercase letter.
- Folder names cannot contain spaces.
- Folder names can only contain letters and numbers.
- Convention is `PascalCase` and is more or less syntactically forced.

## Example directory structure
```
C:
    /System
        /Applications
        /Downloads
        /Trash
        /Temp
    /John
        /Personal
            /Applications
            /Documents
            /Pictures
            /Music
            /Videos
            /Downloads
            /Desktop
            /Trash
            /Temp
        /Work
            /Applications
            /Documents
            /Pictures
            /Music
            /Videos
            /Downloads
            /Desktop
            /Trash
            /Temp
    /Jane
        /Profile1
            /Applications
            /Documents
            /Pictures
            /Music
            /Videos
            /Downloads
            /Desktop
            /Trash
            /Temp
```
