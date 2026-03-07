---
doc_id: concept/fastapi.md/fastapi
chunk_id: concept/fastapi.md/fastapi#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Tokio Spawn vs Spawn Blocking
token_count: 221
summary: # FastAPI Background Tasks and PostgreSQL. To yield a dependency in FastAPI using a Background Task, you must not yield inside the background task itself
---

# FastAPI Background Tasks and PostgreSQL

To yield a dependency in FastAPI using a Background Task, you must not yield inside the background task itself. Instead, yield in the main dependency and pass the background task to the endpoint.

For PostgreSQL connection pools with asyncpg during high concurrency, set `min_size` and `max_size` carefully in the `create_pool` function to avoid connection leaks, and always use an async context manager for the pool.

For handling Pydantic validation errors globally without leaking internal state, override the `RequestValidationError` exception handler and return a custom `JSONResponse` with generic error messages.

# Tokio Spawn vs Spawn Blocking

In Tokio, `tokio::spawn` is for async, non-blocking tasks. It runs on the main async runtime thread pool. `tokio::task::spawn_blocking` is for CPU-bound or blocking I/O workloads, and it runs on a separate dedicated thread pool to avoid starving the async workers.

To implement a graceful shutdown in a Tokio application, use `tokio::select!` with a `CancellationToken` from `tokio_util::sync::CancellationToken`. Await the token's `.cancelled()` method in one branch of the select, and your main work in the other.
