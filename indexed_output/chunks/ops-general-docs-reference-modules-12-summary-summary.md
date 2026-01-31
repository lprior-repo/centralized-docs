---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: A version is considered unstable if its major version is 0 or it has a. pre-release suffix
---


A version is considered unstable if its major version is 0 or it has a
pre-release suffix. Unstable versions are not subject to compatibility
requirements. For example, v0.2.0 may not be compatible with v0.1.0, and
v1.5.0-beta may not be compatible with v1.5.0.

MAJOR VERSION SUFFIXES

Module paths must have a major version
suffix like @v2 that matches the major version. For example, if a module
has the path foo.example/mod@v1 at v1.0.0, it must have the path
foo.example/mod@v2 at version v2.0.0.

Major version suffixes implement the import compatibility
