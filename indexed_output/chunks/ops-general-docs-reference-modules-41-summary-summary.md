---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#41-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary:    reserved file name on Windows [https://learn. com/en-us/windows/win32/fileio/naming-a-file#naming-conventions],
---

   reserved file name on Windows [https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions],
   regardless of case (CON, com1, NuL, and so on).

MODULE CACHING

By default, the cue command caches downloaded modules in the local
filesystem. It uses the local user configuration directory by default, but
that can be changed by setting $CUE_CACHE_DIR, which is
documented under cue help environment [/docs/reference/command/cue-help-environment/].

AUTHORIZATION

For custom OCI registries, CUE understands the usual conventions
