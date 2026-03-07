---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#1-summary
chunk_level: summary
chunk_type: prose
heading: System Overview
token_count: 150
summary: # Architecture Diagrams. Visual representations of the hexagonal architecture and data flow in `doc_transformer`
---

# Architecture Diagrams

Visual representations of the hexagonal architecture and data flow in `doc_transformer`.

## System Overview

```text
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                         DOC TRANSFORMER SYSTEM                       ┃
┃                      Hexagonal Architecture (Rust)                   ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

                                    │
                                    │ CLI Commands
                                    ▼
              ┌─────────────────────────────────────────┐
              │      PRESENTATION LAYER (main.rs)       │
              │  • Parse CLI args (clap)                │
              │  • Display results                      │
              │  • Manage async runtime (tokio)         │
              └──────────────┬──────────────────────────┘
                             │
                             │ Function Calls
                             ▼
┌────────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER (Core Logic)                   │
│                                                                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │transform │  │  chunk   │  │  index   │  │ validate │           │
│  │   .rs    │  │   .rs    │  │   .rs    │  │   .rs    │           │
