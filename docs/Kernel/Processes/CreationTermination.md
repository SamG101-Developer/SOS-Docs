# Process Creation & Termination
## Process Ownership Model
- Each process "owns" its resources, and is responsible for cleaning them up.
- Owned resourced include memory, file descriptors, child processes, etc.
- Each owned resource is tagged with the process ID, so can cleaned up when the process is terminated.

## Process Creation
### Sandboxed Environment & Privileges
- Any process created is created in a sandboxed environment, with no privileges the frist time.
- Every time this process wants to use a privileged operation, it has to request it from the kernel.
- The kernel asks the user for permission to grant the process the requested privilege.
- The user can grant or deny the privilege, and the kernel acts accordingly.
- The updated processes privileges are stored persistently or temporarily, depending on the user's choice.

### Secure Launch Protocol
- The kernel uses a secure launch protocol to ensure that the process is not tampered with.
- The process is signed by the process developer and the user.
- The kernel verifies the signatures before launching the process.
- Checksums are also used to verify the integrity of the process.

## Process Termination
### Graceful Termination
- The process is given a chance to clean up its resources before it is terminated.
- The process is sent a signal to terminate, and it can handle the signal to clean up.
- If the process does not handle the signal, the kernel terminates the process.
- Forceful termination by the kernel cleans up dangling pointers, memory leaks, open file descriptors, orphaned child processes, zombie processes, shared memory segments, message queues, and semaphores, etc.
