# Process Scheduling
## Core Features
- Multi-Level Feedback Queue (MLFQ) scheduling algorithm.
- Fair-share scheduling algorithm.
- Real-time scheduling algorithm for high-priority tasks.
- Energy-aware scheduling.
- Security-centric scheduling.
- Scalability and multi-core utilization.
- TODO: Context switching

## Multi-Level Feedback Queue (MLFQ) Scheduling Algorithm
### Overview
- Multiple queues with different priorities hold processes.
- Higher priority queues get CPU time before lower priority queues.

### Feedback Mechanism
- If a process uses too much CPU time, it is moved to a lower priority queue.
- If a process uses too little CPU time, it is moved to a higher priority queue.

### Time Quantum
- Higher priority queues have smaller time quanta.
- Lower priority queues have larger time quanta.

### Aging
- Processes in lower priority queues are moved to higher priority queues after a certain time.
- This ensures that all processes get a chance to run, preventing starvation.

## Fair-Share Scheduling Algorithm
### Overview
- Each set of MLFQs is assigned to a user.
- The fair-share scheduler allocates CPU time to each user based on their fair share.
- Within each user's MLFQs, the MLFQ scheduler is used to allocate CPU time to processes.

### Weighting
- Each user is assigned a weight, which determines their fair share of CPU time.
- The weight dynamically changes based on the user's CPU usage, system load, and other factors.
- The weight is used to allocate CPU time to each user's MLFQs.

## Real-Time Scheduling Algorithm
### Overview
- High-priority tasks are scheduled using a real-time scheduling algorithm.
- This only applies to the highest 2 priority queues in the MLFQ scheduler.
- Hard real-time tasks are in the 1st highest priority queue, and are guaranteed CPU time.
- Soft real-time tasks are in the 2nd highest priority queue, but are not guaranteed CPU time.

### Priority Inheritance Protocol
- If a high-priority task is blocked by a lower-priority task, the lower-priority task inherits the priority of the high-priority task.
- This ensures that the high-priority task gets CPU time as soon as possible.
- The priority of the lower-priority task is restored once the high-priority task is unblocked.

### Scheduling within Real-Time Queues
- TODO: Earliest Deadline First (EDF), Least Slack Time First (LSTF), Rate Monotonic Scheduling (RMS), etc.

## Energy-Aware Scheduling
### Overview
- The scheduler is aware of the energy consumption of each process.
- The priorities aren't changed by the energy awareness, but the scheduler tries to minimize power consumption.
- Utilizes dynamic voltage and frequency scaling (DVFS) to reduce power consumption.
- TODO

## Security-Centric Scheduling
### Process Isolation
- Hardware assisted virtualization is used to isolate processes (`Intel VT-x`, `AMD-V`).
- Memory protection (`NX` bit, `ASLR`, `DEP`) is used to prevent unauthorized access to memory.

### Resource Access Control
- Sandboxed environment for each process.
- Comprehensive logging of process activities.

### Security-oriented priorities
- Integrate security metrics into the scheduling algorithm for priority assignment.

### Secure Context Switching
- Zero out the registers and portions of the cache before switching to another process.
- Implemented in the kernels context switching code.

### Address Space Layout Randomization (ASLR) for Processes
- Randomly adjust the stack, the heap, and other key memory areas to prevent buffer overflow attacks.

### Trusted Execution Environments (TEEs)
- Provide secure areas of execution for sensitive tasks.
- Separate scheduling for TEEs, isolated from the rest of the system.

### Secure Multi-tenancy
- Ensure that processes from different users are isolated from each other.
- This is done by using different MLFQs for each user.

### Comprehensive Logging of Process Scheduler Activities
- Log all process scheduling activities for auditing and troubleshooting.
- Log the process ID, user ID, priority, time quantum, CPU time, etc.
- Log the reasons for priority changes, aging, feedback, etc.
- Log the energy consumption of each process.
- Log the security metrics used for priority assignment.
- Log the context switching activities.

## Combining the Scheduler Algorithms
- The MLFQ scheduler is used as the base scheduler, forming the backbone of the scheduling system.
- Each user has their own group of MLFQs, and the fair-share scheduler is used to allocate CPU time to each user.
- Real-time scheduling is used for high-priority tasks.
- Energy-aware scheduling is used to reduce power consumption, but doesnt affect priority.
- Security-centric scheduling is integrated into priority assignment.
