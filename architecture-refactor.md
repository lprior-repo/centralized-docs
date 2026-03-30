# Architecture Refactor Summary

**Files Modifed:**
- `src/mcp.rs` (703 lines) was split into multiple files under `src/mcp/` directory to adhere to the `<300 lines` rule.
- `src/cmd/mcp.rs` was reviewed and remained at 6 lines (perfect).

**Splits:**
- `src/mcp/mod.rs`: Public interface and server setup.
- `src/mcp/error.rs`: Domain errors (`CtdMcpError`).
- `src/mcp/types.rs`: Tool parameters, newtypes (`ValidQuery`, `ValidId`, `ValidLimit`), and parameter validation logic.
- `src/mcp/server.rs`: The `CtdMcpServer` and MCP tool handling logic.
- `src/mcp/domain.rs`: New module introducing `IndexData` and other structured domain types to replace the primitive `serde_json::Value` parameter previously used for `ServerState`. This strictly applies Scott Wlaschin's "Parse, don't validate" by building an explicit typed domain model representation for the index.

**Domain-Driven Design (DDD) Enhancements:**
- Replaced `serde_json::Value` (Primitive Obsession) in `ServerState` with a robust `IndexData` type to represent parsed documents and chunks.
- Refactored `CtdMcpError::IndexNotFound` to use a typed `PathBuf` instead of `String`.
- Ensured all tests compile and use the correct new type and module boundaries.