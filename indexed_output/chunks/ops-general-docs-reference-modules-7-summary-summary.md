---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 144
summary:  * The major version suffix declares the major.    version of the module and is of the form @v1 where the version
---

   repository.
 * The major version suffix declares the major
   version of the module and is of the form @v1 where the version
   v1 here must match the major version of the full version it’s been
   published as.

There are also several lexical restrictions on characters allowed in
module paths. As modules are stored in OCI repositories, these correspond
to the restrictions
documented there [https://github.com/opencontainers/distribution-spec/blob/main/spec.md#pulling-manifests].
To summarize:

 * The path must consist of one or more path elements separated by slashes
