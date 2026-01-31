---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: can refuse module paths over 128 characters, and it’s possible to specify. an arbitrary storage prefix, long module paths may fail
---

can refuse module paths over 128 characters, and it’s possible to specify
an arbitrary storage prefix, long module paths may fail.

VERSIONS

A version identifies an immutable snapshot of a module, which may be
either a release or a
pre-release (with a pre-release suffix). Each version starts with the letter
v, followed by a semantic version. See
Semantic Versioning 2.0.0 [https://semver.org/spec/v2.0.0.html] for details on how versions are
formatted, interpreted, and compared.

To summarize, a semantic version consists of three non-negative integers (the
