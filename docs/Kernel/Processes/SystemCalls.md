# System Calls
## System Call Interface
### Access from User Applications
- User applications can request services from the kernel using system calls.
- Given every process is sandboxed, a privilege request is required to access the kernel.
- If the user grants the request, the kernel switches to kernel mode and executes the system call.

### System Call Table
- The system call table is a data structure that contains the addresses of the system call functions.
- The system call table is indexed by the system call number, and contains the address of the system call function.
- The system call table is initialized at boot time, and is read-only.

### System Call Handler
- The system call handler is a function that is called when a system call is made.
- The system call handler is responsible for validating the system call arguments, and executing the system call function.
- The system call handler is executed in kernel mode, and has full access to the hardware.

### System Calls
- There aims to be a small number of system calls, to keep the kernel simple and secure.

#### Process Management

| System Call                                 | Description                          |
|---------------------------------------------|--------------------------------------|
| `fn sys::proc::init(path, args)`            | Create a new process.                |
| `fn sys::proc::exit(exit_code)`             | Exit the current process.            |
| `fn sys::proc::wait(pid)`                   | Wait for a process to exit.          |
| `fn sys::proc::kill(pid)`                   | Kill a process.                      |
| `fn sys::proc::list()`                      | List all processes.                  |
| `fn sys::proc::suspend(pid)`                | Suspend a process.                   |
| `fn sys::proc::resume(pid)`                 | Resume a process.                    |
| `fn sys::proc::signal(pid, sig)`            | Send a signal to a process.          |
| `fn sys::proc::set_priority(pid, priority)` | Set the priority of a process.       |
| `fn sys::proc::set_affinity(pid, affinity)` | Set the CPU affinity of a process.   |
| `fn sys::proc::set_env(pid, key, value)`    | Set an environment variable.         |
| `fn sys::proc::set_cwd(pid, path)`          | Set the current working directory.   |
| `fn sys::proc::get_priority(pid)`           | Get the priority of a process.       |
| `fn sys::proc::get_affinity(pid)`           | Get the CPU affinity of a process.   |
| `fn sys::proc::get_env(pid, key)`           | Get an environment variable.         |
| `fn sys::proc::get_cwd(pid)`                | Get the current working directory.   |
| `fn sys::proc::get_pid()`                   | Get the PID of the current process.  |
| `fn sys::proc::get_ppid()`                  | Get the PPID of the current process. |
| `fn sys::proc::get_uid(pid)`                | Get the UID of a process.            |
| `fn sys::proc::get_info(pid)`               | Get information about a process.     |

#### Memory Management

| System Call                                 | Description              |
|---------------------------------------------|--------------------------|
| `fn sys::mem::alloc(size)`                  | Allocate memory.         |
| `fn sys::mem::free(ptr)`                    | Free memory.             |
| `fn sys::mem::realloc(ptr, size)`           | Reallocate memory.       |
| `fn sys::mem::map(ptr, size, flags)`        | Map memory.              |
| `fn sys::mem::unmap(ptr)`                   | Unmap memory.            |
| `fn sys::mem::info()`                       | Get memory information.  |
| `fn sys::mem::protect(ptr, size, flags)`    | Protect memory.          |
| `fn sys::mem::lock(ptr, size)`              | Lock memory.             |
| `fn sys::mem::unlock(ptr, size)`            | Unlock memory.           |
| `fn sys::mem::copy(dst, src, size)`         | Copy memory.             |
| `fn sys::mem::move(dst, src, size)`         | Move memory.             |
| `fn sys::mem::compare(ptr1, ptr2, size)`    | Compare memory.          |

#### File Management

| System Call                                  | Description            |
|----------------------------------------------|------------------------|
| `fn sys::fs::open(path, flags)`              | Open a file.           |
| `fn sys::fs::close(fd)`                      | Close a file.          |
| `fn sys::fs::read(fd, buf, len)`             | Read from a file.      |
| `fn sys::fs::write(fd, buf, len)`            | Write to a file.       |
| `fn sys::fs::seek(fd, offset)`               | Seek in a file.        |
| `fn sys::fs::info(path)`                     | Get file/dir info.     |
| `fn sys::fs::list(path)`                     | List directory.        |
| `fn sys::fs::create(path)`                   | Create a file/dir.     |
| `fn sys::fs::remove(path)`                   | Remove a file/dir.     |
| `fn sys::fs::rename(old, new)`               | Rename a file/folder.  |
| `fn sys::fs::link(old, new)`                 | Create a link.         |
| `fn sys::fs::unlink(path)`                   | Remove a link.         |
| `fn sys::fs::copy(src, dst)`                 | Copy a file/dir.       |
| `fn sys::fs::move(src, dst)`                 | Move a file/dir.       |
| `fn sys::fs::protect(path, flags)`           | Protect a file/dir.    |
| `fn sys::fs::lock(path)`                     | Lock a file/dir.       |
| `fn sys::fs::unlock(path)`                   | Unlock a file/dir.     |
| `fn sys::fs::mount(path, fs)`                | Mount a file system.   |
| `fn sys::fs::unmount(path)`                  | Unmount a file system. |
| `fn sys::fs::format(path, fs)`               | Format a file system.  |
| `fn sys::fs::get_metadata(path)`             | Get file metadata.     |
| `fn sys::fs::set_metadata(path, key, value)` | Set file metadata.     |

#### Device Management

| System Call                                  | Description            |
|----------------------------------------------|------------------------|
| `fn sys::dev::open(path, flags)`             | Open a device.         |
| `fn sys::dev::close(fd)`                     | Close a device.        |
| `fn sys::dev::read(fd, buf, len)`            | Read from a device.    |
| `fn sys::dev::write(fd, buf, len)`           | Write to a device.     |
| `fn sys::dev::info(path)`                    | Get device info.       |
| `fn sys::dev::list()`                        | List devices.          |
