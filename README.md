# Nexus: Autonomous Data Ingestion & Analytics Engine

I built an asynchronous, multi-threaded backend system built in Rust, a language I am trying to learn and master. This was a great way to get hands on experience with Backend systems. Nexus autonomously harvests live atmospheric data, manages thread-safe in-memory state, and provides real-time analytical aggregations via a REST API. 

## Architecture

I built Nexus to see this core backend principles in actiaon and understand them in a more hands on way:
- **Concurrency & State Management:** Utilizes `Arc` (Atomic Reference Counting) and `Mutex` (Mutual Exclusion) to safely share a dynamic data vault across multiple threads without race conditions.
- **Asynchronous I/O:** Powered by the `tokio` runtime to handle non-blocking HTTP requests and background worker loops simultaneously.
- **Autonomous Workers:** A detached background process polls third-party APIs (Open-Meteo) at standard intervals, writing to the shared state while the main thread serves clients.
- **Strict Typing & Deserialization:** Leverages `serde` to rigorously map messy, real-world JSON into memory-safe Rust structs.

## Tech Stack
- **Language:** Rust
- **Framework:** Axum (Routing)
- **Runtime:** Tokio (Asynchronous I/O)
- **HTTP Client:** Reqwest
- **Serialization:** Serde

## Endpoints

| Method | Route | Description |
| :--- | :--- | :--- |
| `GET` | `/` | System health check. Returns backend pulse. |
| `GET` | `/vault` | Dumps the current raw JSON state of the in-memory database. |
| `GET` | `/analyze` | Computes and returns real-time statistics (total records, average temperature, max windspeed) across all ingested data. |

## How to Run

1. Ensure you have the Rust toolchain installed.
2. Clone this repository.
3. Run `cargo run`.
4. The automaton will immediately begin fetching data every 5 seconds.
5. Navigate to `http://localhost:3000/analyze` to view the live aggregations.

## Quick Note 

I don't suggest running the server for too long. otherwise, the Automaton will work long enough for the server to run out of RAM. That won't be very fun. Anyways, once I learn Database, i'll improve the Vault to be an actual DBMS. for now, this is it. I won't be making the UI any time soon either. 
