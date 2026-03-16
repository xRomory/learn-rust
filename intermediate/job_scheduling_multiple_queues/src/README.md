# Multiple Queue Scheduling

## Multi-level Queue (MLQ) Scheduling

In **Multi-level Queue Scheduling**, the operating system (OS)
separates jobs into different queues based on job type or priority.

Each queue:
- has its own scheduling algorithm
- has a priority relative to other queues.

So scheduling happens in two levels:
1. Between queues (which queue gets the CPU)
2. Inside a queue (which job in that queue runs)

### 1. Jobs Are Classified Into Queues
When a job enters the system, it is assigned to a specific queue.

This assignment is **not random**. It is determined by by predefined rules.

Typical classification:
| Job Type         | Queue   |
| ---------------- | ------- |
| System processes | Queue 0 |
| Interactive jobs | Queue 1 |
| Batch jobs       | Queue 2 |
| Background jobs  | Queue 3 |


Example:

```
Queue 0 (Highest priority) → System
Queue 1 → Interactive
Queue 2 → Batch
Queue 3 (Lowest priority) → Background
```

So when a job arrives, the OS decides:

```
Job → Which queue should it go to?
```

Example:

| Job | Type        | Assigned Queue |
| --- | ----------- | -------------- |
| J1  | Interactive | Q1             |
| J2  | Background  | Q3             |
| J3  | System      | Q0             |

So the priority is determined by the queue the job is placed in.

### 2. Each Queue Uses Its Own Algorithm
Each queue can use **different scheduling algorithms**.

Example configuration:
| Queue | Algorithm                      |
| ----- | ------------------------------ |
| Q0    | Round Robin (short time slice) |
| Q1    | Round Robin                    |
| Q2    | HRRN                           |
| Q3    | FCFS                           |

So if a job is in Queue 3, it will **always follow FCFS**.

A job inherits the algorithm of the queue it belongs to.

Example:
```
J1 → Q1 → Round Robin
J2 → Q3 → FCFS
J3 → Q0 → Round Robin
```

### 3. CPU Chooses Which Queue Runs
The scheduler always checks **queues by priority order**.

Example:

```
Priority order:

Q0
Q1
Q2
Q3
```

Scheduler logic:
```
Check Q0
If empty → check Q1
If empty → check Q2
If empty → check Q3
```

The CPU never runs a lower queue if higher queue has jobs.

### 4. Why Multiple Queue Scheduling
This system allows the OS to prioritize different workloads:

| Job Type    | Desired Behavior    |
| ----------- | ------------------- |
| Interactive | fast response       |
| System      | immediate execution |
| Batch       | efficiency          |
| Background  | lowest priority     |

Example:
```
Mouse click event → high priority queue
Video rendering → background queue
```

Without the system, a large batch job could freeze interactive programs.

## Multi-level Feedback Queue (MLFQ) Scheduling
MLFQ improves MLQ by allowing jobs to move between queues, which makes the system adaptive.

### Characteristics of MLFQ

Queues still exists, but jobs can change priority depending on their behavior.

- Processes can move between queues based on their CPU usage or I/O behavior.
- Processes are **not** permanently assigned to any queue.
- Priority changes dynamically depending on process performance.