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

## File Management
```
FileCreate(dir: Str) -> Result<(), FileErr>;
FileDelete(dir: Str, force: Bool) -> Result<(), FileErr>;
FileOpen(dir: Str, flags: U64) -> Result<U64, FileErr>;
FileClose(fid: U64) -> Result<(), FileErr>;
FileWrite(fid: U64, data: U8[]) -> Result<U64, FileErr>;
FileRead(fid: U64, size: U64) -> Result<U8[], FileErr];
FileSeek(fid: U64, offset: U64) -> Result<U64, FileErr>;
FileList(dir: Str) -> Str[];
FileRename(old: Str, new: Str) -> Result<(), FileErr>
FileCopy(src: Str, dst: Str);
FileMove(src: Str, dst: Str);
FileProtect(file: Str, permissions: U64);
FileProtection(file: Str) -> U64;
FileLock(file: Str);
FileUnlock(file: Str);
FileLink(file: Str, link: Str);
FileUnlink(link: Str);
FileGetMetadata(file: Str) -> Metadata;
FileSetMetadata(file: Str, metadata: Metadata);
FileMount(drive: Str, fs: Str);
FileUnmount(drive: Str);
FileFormat(drive: Str, fs: Str);
FileTruncate(file: Str, size: U64);
```

## Process Management
```
ProcessCreate(path: Str, args: Str[]) -> Result<U64, ProcessErr>;
ProcessExit(pid: U64) -> Result<U64, ProcessErr>;
ProcessWait(pid: U64) -> Result<U64, ProcessErr>;
ProcessCopy(pid: U64) -> Result<Process, ProcessErr>;
ProcessKill(pid: U64);
ProcessList() -> U64[];
ProcessSuspend(pid: U64);
ProcessResume(pid: U64);
ProcessSignal(pid: U64, signal: U64);
ProcessSetPriority(pid: U64, priority: U64);
ProcessSetAffinity(pid: U64, affinity: U64);
ProcessSetEnv(pid: U64, key: Str, value: Str);
ProcessSetCwd(pid: U64, path: Str);
ProcessGetPriority(pid: U64) -> U64;
ProcessGetAffinity(pid: U64) -> U64;
ProcessGetEnv(pid: U64, key: Str) -> Result<Str, ProcessErr>;
ProcessGetCwd(pid: U64) -> Str;
ProcessGetPid() -> U64;
ProcessGetPpid() -> U64;
ProcessGetUid(pid: U64) -> U64;
ProcessGetStatus(pid: U64) -> U64;
ProcessGetInfo(pid: U64) -> ProcessInfo;
```

## Memory Management
```
MemoryAlloc(size: U64) -> Result<U64, MemoryErr>;
MemoryFree(ptr: U64);
MemoryRealloc(ptr: U64, size: U64) -> Result<U64, MemoryErr>;
MemoryMap(ptr: U64, size: U64, flags: U64) -> Result<U64, MemoryErr>;
MemoryUnmap(ptr: U64);
MemoryInfo() -> MemoryInfo;
MemoryProtect(ptr: U64, size: U64, flags: U64);
MemoryLock(ptr: U64, size: U64);
MemoryUnlock(ptr: U64, size: U64);
MemoryCopy(dst: U64, src: U64, size: U64);
MemoryMove(dst: U64, src: U64, size: U64);
MemoryCompare(ptr1: U64, ptr2: U64, size: U64) -> U64;
MemoryPin(ptr: U64, size: U64);
MemoryUnpin(ptr: U64, size: U64);
```

## Inter-Process Communication
```
SharedMemoryCreate(size: U64) -> Result<U64, SharedMemoryErr>;
SharedMemoryDestroy(sid: U64);
SharedMemoryAttach(sid: U64, pid: U64, signed_key: Sig) -> Result<(), SharedMemoryErr>;
SharedMemoryDetach(sid: U64, pid: U64) -> Result<(), SharedMemoryErr>;
SharedMemoryWrite(sid: U64, data: U8[], persistent: Bool) -> Result<U64, SharedMemoryErr>;
SharedMemoryRead(sid: U64, size: U64) -> Result<U8[], SharedMemoryErr>;
SharedMemoryACLUpdate(sid: U64, pid: U64, permissions: U64) -> Result<(), SharedMemoryErr>;
SharedMemoryLock(sid: U64);
SharedMemoryUnlock(sid: U64);
```

## Device Management
```
DeviceOpen(?: ?, access_mode: U64) -> Result<U64, DeviceErr>;
DeviceClose(did: Device) -> Result<(), DeviceErr>;
DeviceRead(did: Device, size: U64) -> Result<U8[], DeviceErr>;
DeviceWrite(did: Device, data: U8[]) -> Result<U64, DeviceErr>;
DeviceIoctl(did: Device, command: U64, arg: U64) -> Result<U64, DeviceErr>;
DeviceSelect(did: Device, timeout: U64) -> Result<U64, DeviceErr>;
DeviceMap(did: Device, size: U64, flags: U64) -> Result<U64, DeviceErr>;
DeviceUnmap(did: Device, ptr: U64);
DeviceInterruptEnable(did: Device);
DeviceInterruptDisable(did: Device);
DeviceSynchronize(did: Device);
DeviceStatus(did: Device) -> U64;
DeviceList() -> U64[];
```


## Network Management
```
SocketCreate(domain: U64, type: U64, protocol: U64) -> Result<U64, SocketErr>;
SocketClose(sid: U64) -> Result<(), SocketErr>;
SocketBind(sid: U64, address: Str, port: U64) -> Result<(), SocketErr>;
SocketListen(sid: U64, backlog: U64) -> Result<(), SocketErr>;
SocketAccept(sid: U64) -> Result<U64, SocketErr>;
SocketConnect(sid: U64, address: Str, port: U64) -> Result<(), SocketErr>;
SocketSend(sid: U64, data: U8[]) -> Result<U64, SocketErr>;
SocketReceive(sid: U64, size: U64) -> Result<U8[], SocketErr>;
SocketSendTo(sid: U64, data: U8[], address: Str, port: U64) -> Result<U64, SocketErr>;
SocketReceiveFrom(sid: U64, size: U64) -> Result<(U8[], Str, U64), SocketErr>;
SocketSetOption(sid: U64, level: U64, option: U64, value: U64) -> Result<(), SocketErr>;
SocketGetOption(sid: U64, level: U64, option: U64) -> Result<U64, SocketErr>;
```
