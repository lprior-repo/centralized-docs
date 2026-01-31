---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#140-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: // eliminated from the disjunction. MyList: #List & { head: 1, tail: { head: 2 }}
---

// eliminated from the disjunction.
MyList: #List & { head: 1, tail: { head: 2 }}

MODULES, INSTANCES, AND PACKAGES

CUE configurations are constructed combining instances.
An instance, in turn, is constructed from one or more source files belonging
to the same package that together declare the data representation.
Elements of this data representation may be exported and used
in other instances.

SOURCE FILE ORGANIZATION

Each source file consists of an optional package clause defining collection
of files to which it belongs,
