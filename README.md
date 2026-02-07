# Multithreaded Rust Web Server


**Luke Ramirez**
## Overview

- This project is a simple web server that can run with multiple threads
- It depicts the rust language 
- This program uses worker threads to run connection handling for a server. It uses a thread pool.

---

## Goals of the Project

- Run multiple connections as once
- Compile correcty


---

## Technologies & logic Used

- **Rust**
- **Thread Pool**
- **Message Passing (mpsc channels)**
- **Concurrency & Synchronization**

---

## Project Architecture

### ThreadPool

- Why use a thread pool instead of spawning threads per request? Thread pool was used here to avoid having to spawn each thread individually. 
- jobs are submitted through the mpsc channel



---

### Worker

- Workers are all spawned at the beggining. 
- They wait or idle until a job is sent to them via Channel
- Shut down is handled via a function with termination messeging. Enum is used for job/worker Termination. 

---

## Concurrency & Thread Safety
Explain how this project avoids data races and unsafe behavior.

- How ownership is enforced
- Why `Mutex` is needed
- Why jobs must be `Send + 'static`
- Where locking occurs and why

---

## Web Server Flow
Describe the lifecycle of a single HTTP request.

1. Listener accepts a connection
2. Connection is passed to the thread pool
3. Worker receives and executes the job
4. Request is parsed
5. Response is generated and sent

---


## Main program concepts

- Rust ownership & borrowing
- Concurrency patterns
- Thread pools
- Networking basics

---

## How to Run the Project\

simple program explination video

[https://youtu.be/aN9HrkzQZl4]()

```bash
cargo run

# the use of the local web url

127.0.0.1:7878