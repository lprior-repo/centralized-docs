---
doc_id: usage-example
chunk_id: usage-example#19
chunk_level: summary
chunk_type: prose
heading: Production Deployment
token_count: 69
summary: WORKDIR /app. FROM debian:bookworm-slim
---



WORKDIR /app
COPY . .

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mcp_server /usr/local/bin/
COPY indexed_output /app/indexed_output
WORKDIR /app
CMD ["mcp_server"]
```

Build and run:
```bash
docker build -t mcp-server .
docker run -i mcp-server
```

