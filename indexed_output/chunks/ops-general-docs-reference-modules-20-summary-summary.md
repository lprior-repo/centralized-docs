---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#20-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: The optional modulePrefix specifes that all modules with a path that. has the given prefix will use the associated registry
---


The optional modulePrefix specifes that all modules with a path that
has the given prefix will use the associated registry. If there are
multiple registries with a prefix, the longest matching prefix wins.
It’s an error for there to be multiple entries with the same prefix.

The hostname holds the OCI registry host (in square brackets if it’s
an IPv6 address), with an optional numeric TCP port.

Each module is stored inside its own repository in the registry which
is named after the module path. The repoPrefix holds a prefix to be
