---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#47-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: the minor and patch versions must be set to 0.  Semantic versions with major
---

the minor and patch versions must be set to 0. Semantic versions with major
version 0 are considered unstable.

major version suffix: A module path suffix that matches the major version
number. For example, @v2 in foo.example/mod@v2. See
the section on Major version suffixes [/docs/reference/modules/#major-version-suffixes].

minimal version selection (MVS): The algorithm used to determine the
versions of all modules that will be used in a build. See the section on
Minimal version selection [/docs/reference/modules/#minimal-version-selection] for details.
