---
doc_id: concept/fastapi.md/fastapi
chunk_id: concept/fastapi.md/fastapi#0-detailed
chunk_level: detailed
chunk_type: prose
heading: FastAPI Background Tasks and PostgreSQL
token_count: 112
summary: # FastAPI Background Tasks and PostgreSQL. To yield a dependency in FastAPI using a Background Task, you must not yield inside the background task itself
---

# FastAPI Background Tasks and PostgreSQL

To yield a dependency in FastAPI using a Background Task, you must not yield inside the background task itself. Instead, yield in the main dependency and pass the background task to the endpoint.

For PostgreSQL connection pools with asyncpg during high concurrency, set `min_size` and `max_size` carefully in the `create_pool` function to avoid connection leaks, and always use an async context manager for the pool.

For handling Pydantic validation errors globally without leaking internal state, override the `RequestValidationError` exception handler and return a custom `JSONResponse` with generic error messages.

