---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: configurations. Where most validation systems are limited to checking whether a concrete
---

configurations.
Where most validation systems are limited to checking whether a concrete
value matches a schema, CUE can validate whether any instance of
one schema is also an instance of another (is it backwards compatible?),
or compute a new schema that represents all instances that match
two other schema.

HISTORY

Although it is a very different language, the roots of CUE lie in GCL,
the dominant configuration language in use at Google as of this writing.
It was originally designed to configure Borg, the predecessor of Kubernetes.
