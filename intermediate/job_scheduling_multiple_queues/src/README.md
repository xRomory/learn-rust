# Multiple Queue Scheduling

In **Multi-level Queue Scheduling**, the operating system (OS)
separates jobs into different queues based on job type or priority.

Each queue:
- has its own scheduling algorithm
- has a priority relative to other queues.

So scheduling happens in two levels:
1. Between queues (which queue gets the CPU)
2. Inside a queue (which job in that queue runs)

## 1. Jobs Are Classified Into Queues
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

## 2. Each Queue Uses Its Own Algorithm
Each queue can use **different scheduling algorithms**.

Example configuration:
| Queue | Algorithm                      |
| ----- | ------------------------------ |
| Q0    | Round Robin (short time slice) |
| Q1    | Round Robin                    |
| Q2    | HRRN                           |
| Q3    | FCFS                           |

So if a job is in Queue 3, it will **always follow FCFS**.

This answers the question:
> is it not definite what algorithm will be implemented to each job?

**It is definite**

A job inherits the algorithm of the queue it belongs to.

Example:
```
J1 → Q1 → Round Robin
J2 → Q3 → FCFS
J3 → Q0 → Round Robin
```

## 3. CPU Chooses Which Queue Runs
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