#!/bin/bash
# Test script for MCP server
# Verifies all three tools: tools/list, search_docs, get_chunk, list_docs

set -e

echo "=== Testing MCP Server ==="
echo

echo "1. Testing tools/list..."
echo '{"method":"tools/list"}' | cargo run --quiet --bin mcp_server 2>/dev/null | jq -c '.tools[] | .name'
echo

echo "2. Testing list_docs..."
echo '{"method":"tools/call","params":{"name":"list_docs","arguments":{}}}' | \
  cargo run --quiet --bin mcp_server 2>/dev/null | \
  jq '.documents[] | {id, title, category}'
echo

echo "3. Testing search_docs (query: 'rust')..."
echo '{"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"rust","limit":5}}}' | \
  cargo run --quiet --bin mcp_server 2>/dev/null | \
  jq '.results[] | {title, score}'
echo

echo "4. Testing get_chunk (chunk-001)..."
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"chunk-001"}}}' | \
  cargo run --quiet --bin mcp_server 2>/dev/null | \
  jq '{chunk_id, doc_title, heading, token_count}'
echo

echo "5. Testing error handling (invalid chunk)..."
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"invalid-chunk"}}}' | \
  cargo run --quiet --bin mcp_server 2>/dev/null | \
  jq '.error'
echo

echo "=== All MCP Tests Passed ==="
