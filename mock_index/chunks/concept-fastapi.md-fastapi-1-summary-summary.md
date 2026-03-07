---
doc_id: concept/fastapi.md/fastapi
chunk_id: concept/fastapi.md/fastapi#1-summary
chunk_level: summary
chunk_type: prose
heading: Tokio Spawn vs Spawn Blocking
token_count: 137
summary: For handling Pydantic validation errors globally without leaking internal state, override the `RequestValidationError` exception handler and return a custom `JSONResponse` with generic error messag...
---




For handling Pydantic validation errors globally without leaking internal state, override the `RequestValidationError` exception handler and return a custom `JSONResponse` with generic error messages.

# Tokio Spawn vs Spawn Blocking

In Tokio, `tokio::spawn` is for async, non-blocking tasks. It runs on the main async runtime thread pool. `tokio::task::spawn_blocking` is for CPU-bound or blocking I/O workloads, and it runs on a separate dedicated thread pool to avoid starving the async workers.

To implement a graceful shutdown in a Tokio application, use `tokio::select!` with a `CancellationToken` from `tokio_util::sync::CancellationToken`. Await the token's `.cancelled()` method in one branch of the select, and your main work in the other.
