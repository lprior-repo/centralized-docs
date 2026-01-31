---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary:  * The minor version must be incremented and the patch.    version set to zero after a backwards compatible change, for example, after a
---

 * The minor version must be incremented and the patch
   version set to zero after a backwards compatible change, for example, after a
   new function is added.
 * The patch version must be incremented after a change
   that does not affect the module’s public interface, such as a bug fix or
   change to the documentation.
 * The pre-release suffix indicates a version is a pre-release. Pre-release versions sort before
   the corresponding release versions. For example, v1.2.3-pre comes before
   v1.2.3.
