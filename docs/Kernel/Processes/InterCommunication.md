# Process Inter Communication
## Core Features
- Shared memory for fast data exchange
- Key generation in private memory, exchanged through shared memory
- Shared messaged e2e encrypted and placed in shared memory

## Shared Memory
### Overview
- A shared memory segment is generated to exchange data between two processes.
- A signed key exchange is performed to generate an encryption key for messages.
- Encrypted messages are placed in the shared memory segment.
- Any number of processes can be added to the shared memory segment.


### Shared Memory Initialization
- A process creates a shared memory segment.
- The shared memory process ID is stored in the shared memory segment.
- All signatures must include the shared memory process ID, preventing replay attacks.


### Key Generation & Exchange
- When a new process X joins the shared memory:
  - Process X creates an ephemeral key pair.
  - Process X signs the ephemeral public key with its static private key.
  - Process X places the signed ephemeral public key in the shared memory segment.
  - This is a non-consumable message (stays in shared memory until process X leaves).
- When Process Y joins the shared memory:
  -  Process Y reads all the signed ephemeral public keys from the shared memory segment.
  - Process Y verifies the signatures using the static public keys of all processes.
  - Process Y encapsulates a unique master key with each ephemeral public key.
  - Process Y signs the encapsulated master keys with its static private key.
  - Process Y places the signed encapsulated master keys in the shared memory segment.
- The other processes (including Process X) consume the signed encapsulated master keys directed at them.
  - They verify the signature using the static public keys of Process Y.
  - They decapsulate the master key with their own ephemeral private key.
- All processes now have the same master key for symmetric encryption with Process Y.
- Symmetric keys are rotated with AES key wrapping and control messages.
- Not all processes will necessarily exchange keys with each other, but have the ability too at any time.

### Message Exchange
- When a process wants to send a message to every process, it uses a modified multicast algorithm.
  - It individually encrypts n replicas of the message with each process' shared secret.
  - It places the encrypted messages in the shared memory segment.
- Each process then takes their message from the shared memory segment.
  - They decrypt the message with their shared secret.
  - They process the message and respond with a message of their own, probably with a unicast.
- The flexibility allows processes to send messages to only a subset of processes if desired.

### Message types
- Non-consumable messages: These stay in shared memory until the sender process leaves the shared memory.
- Consumable messages: These are removed from shared memory after being read by the recipient process.

### Shared Memory Access Control
- A shared memory access control list is used to ensure that only the intended processes can access the shared memory segment.
- The shared memory access control list is immutable to everyone except the process that created the shared memory segment.
- If a process is added to the shared memory access control list, it is signed by the process that created the shared memory segment.
- Key exchanges can be performed between the new process when it joins, and any of the other processes.
