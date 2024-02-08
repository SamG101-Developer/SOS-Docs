# Memory Protection
## Hardware Memory Protection
### Memory Protection Registers
- Use base and limit registers to define legal memory regions.
- Check every address generated by the CPU to ensure it is within the legal range.
- Page table protection bits are used to define the access rights for each page.

### Memory Management Unit (MMU)
- The MMU is a hardware unit that is responsible for translating virtual addresses to physical addresses.
- Enforces page bit restrictions, and checks the access rights for each page.

## Access Control
### User vs Kernel Mode
- User mode is used for user applications, and kernel mode is used for the kernel.
- User mode has fewer privileges than kernel mode, and isolates user applications from the kernel.
- Kernel mode has full access to the hardware, and can execute privileged instructions.

#### System Call Interface
- User applications can request services from the kernel using system calls.
- Given every process is sandboxed, a privilege request is required to access the kernel.
- If the user grants the request, the kernel switches to kernel mode and executes the system call.

### Segmentation
- Separate the address space into multiple segments, and assign access rights to each segment.
- Example: Code segment, data segment, stack segment.
- Different protection bits are used to define the access rights for each segment.

### Ring Protection

## Software Memory Protection
### Sandboxing
- Process sandboxing extends to memory protection.
- Each process has its own address space, and cannot access the memory of other processes.
- Sandbox privilege control enforces memory range restrictions.

### Memory Protection Keys (MPK)
- Each region of memory is assigned a protection key.
- The protection key is set of bits that define the access rights for the memory region.
- The protection key is checked by the CPU for every memory access.

### Address Space Layout Randomization (ASLR)
- Base address randomization is used to randomize the base address of the stack, heap, libraries and executable.
- This makes it harder for attackers to predict the memory layout of a process.

### Kernel Address Space Layout Randomization (KASLR)
- Randomize the base address of the kernel to make it harder for attackers to predict the memory layout of the kernel.
- This makes it harder for attackers to exploit kernel vulnerabilities.