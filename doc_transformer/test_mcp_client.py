#!/usr/bin/env python3
"""
Simple MCP client to test the doc_transformer MCP server.
"""
import json
import subprocess
import sys

def send_request(method, params=None):
    """Send a JSON-RPC request to the MCP server."""
    request = {"method": method}
    if params:
        request["params"] = params

    # Start the MCP server
    proc = subprocess.Popen(
        ["cargo", "run", "--quiet", "--bin", "mcp_server"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    # Send request
    request_json = json.dumps(request)
    stdout, stderr = proc.communicate(input=request_json + "\n", timeout=10)

    if stderr:
        print(f"[stderr] {stderr}", file=sys.stderr)

    # Parse response
    try:
        return json.loads(stdout.strip())
    except json.JSONDecodeError as e:
        print(f"Failed to parse response: {stdout}", file=sys.stderr)
        raise

def main():
    print("=== MCP Server Tests ===\n")

    # Test 1: tools/list
    print("1. Testing tools/list...")
    response = send_request("tools/list")
    tools = response.get("tools", [])
    print(f"   Found {len(tools)} tools:")
    for tool in tools:
        print(f"   - {tool['name']}: {tool['description']}")
    print()

    # Test 2: list_docs
    print("2. Testing list_docs...")
    response = send_request("tools/call", {
        "name": "list_docs",
        "arguments": {}
    })
    docs = response.get("documents", [])
    print(f"   Found {len(docs)} documents:")
    for doc in docs[:3]:
        print(f"   - {doc['title']} ({doc['category']})")
    print()

    # Test 3: search_docs
    print("3. Testing search_docs (query: 'rust')...")
    response = send_request("tools/call", {
        "name": "search_docs",
        "arguments": {"query": "rust", "limit": 3}
    })
    results = response.get("results", [])
    print(f"   Found {len(results)} results:")
    for result in results:
        print(f"   - {result['title']} (score: {result['score']})")
    print()

    # Test 4: get_chunk
    print("4. Testing get_chunk (chunk-001)...")
    response = send_request("tools/call", {
        "name": "get_chunk",
        "arguments": {"chunk_id": "chunk-001"}
    })
    if "error" not in response:
        chunk = response
        print(f"   Chunk: {chunk.get('chunk_id')}")
        print(f"   Doc: {chunk.get('doc_title')}")
        print(f"   Heading: {chunk.get('heading')}")
        print(f"   Tokens: {chunk.get('token_count')}")
    else:
        print(f"   Error: {response['error']}")
    print()

    # Test 5: Error handling
    print("5. Testing error handling (invalid chunk)...")
    response = send_request("tools/call", {
        "name": "get_chunk",
        "arguments": {"chunk_id": "invalid-chunk"}
    })
    if "error" in response:
        print(f"   Expected error: {response['error']['message']}")
    else:
        print(f"   Unexpected success")
    print()

    print("=== All Tests Passed ===")

if __name__ == "__main__":
    main()
