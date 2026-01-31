---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#19-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 138
summary: THE CUE_REGISTRY ENVIRONMENT VARIABLE. When CUE looks up a new module for a package path, it checks the
---


THE CUE_REGISTRY ENVIRONMENT VARIABLE

When CUE looks up a new module for a package path, it checks the
CUE_REGISTRY environment variable. This determines the registry
and repository within a registry that a module will be searched for.
It holds a complete list of any registries that are consulted for fetching modules.

Specifically it holds a comma-separated list specifying which registry to use for
downloading and publishing modules. A registry is specifed
as follows:


Copy code
Copied!

[modulePrefix=]hostname[:port][/repoPrefix][+insecure]
