# Snapshotting and Versioning
## Overview
- Create snapshots of the file system to capture the state of the file system at a specific point in time.
- The entire file isn't saved, only the changes that have been made since the last snapshot.
- Works like GitHub, where each change is a commit, and the current file can be rolled back to any previous state.

## Commit & Push
- Every change made is committed to the snapshot store in the file system.
- Once idle for a short amount of time, pending commits are pushed to the snapshot store.
- The latest snapshot is pulled to the current version saved on the disk.

## Rollback
- Rollback to a previous version of the file system by reverting to a previous snapshot.
- Previews and timestamps can be viewed from the snapshot metadata.

## Branching
- Editing an old snapshot causes a branch in the file system, and a new snapshot is created.
- It doesn't remove the other snapshots following the old snapshot.
- The tree system and associated metadata can be viewed in a tree-like structure.
