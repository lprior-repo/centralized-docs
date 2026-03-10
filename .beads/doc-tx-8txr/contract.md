# Contract: doc-tx-8txr

## bead_id: doc-tx-8txr
## bead_title: Add real semantic embeddings (OpenAI/Cohere) to HNSW index
## phase: p1
## updated_at: 2026-03-01T20:55:00Z

---

## Problem Statement

Add support for real semantic embeddings from OpenAI or Cohere API to the HNSW index.

## Preconditions

- OpenAI or Cohere API key available

## Postconditions

- HNSW index contains semantic embeddings from API
- Exit code 1 on API errors

## Acceptance Tests

- Valid API key: embeddings generated
- Invalid API key: exit 1 with error

## Verification

Test with real API key and verify vectors differ from random
