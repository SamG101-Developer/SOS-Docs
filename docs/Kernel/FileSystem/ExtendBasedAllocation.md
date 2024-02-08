# Extend-Based Allocation & Chunking with Deduplication
## Overview
- Extent-based allocation is used to allocate space for files.
- Break files into fixed-size chunks, and store them in a chunk store.
- Deduplicate chunks to save space, and reduce storage costs.

## Store a chunk
- Create chunk identifiers by hashing the chunk data, and use them as keys in the chunk store.
- If the chunk identifier is not found in the chunk store, store the chunk data, and return the chunk identifier.
- Otherwise, just return the existing chunk identifier.
- Use a reference count to track the number of references to each chunk and manage their lifecycle.
- When a chunk has no references, remove it from the chunk store.

## Chunk stores
- There is a chunk store per user, to ensure separation of data.
- There is a system chunk store for system files.

## Chunk size determination
- Variable sized chunks are used to improve deduplication.
- Use a rolling hash to identify similar chunks.

## Deduplication
- Compare the hash of each chunk with the hashes of existing chunks.
- If a match is found, the chunk is deduplicated, and the reference count is incremented.
- If no match is found, the chunk is compressed and then stored in the chunk store, and a new chunk identifier is created.

## Compression
- Compression is done after hashing, to avoid the overhead of compressing chunks that are already in the chunk store.
- Compression uses the `Zstandard` algorithm, which is fast and efficient for small chunks.

## Reconstruct a file
- Reconstruct a file by retrieving the chunks from the chunk store, and concatenating them in the correct order.

## Security
- Each chunk is individually encrypted, and the chunk store is encrypted at rest.
- Unique keys are derived from the system-wide encryption, a chunk identifier, time, a salt and version number.
- Chunks at rest are re-encrypted periodically by re-salting and re-encrypting the chunk data.

## Performance
- A "hot storage area" is used to store the most frequently accessed chunks, to improve performance.
- Hot storage areas are stored on the RAM for quick access.
- Recent and frequently accessed chunks are stored in the hot storage area, and are quickly accessible.
- RAM performance isn't affected because the hot storage area is a small portion of the total RAM.

## Metadata
- Store metadata for each chunk, including the chunk identifier, reference count, and salt.
- Versioning information is also stored to track changes to the chunk data.
