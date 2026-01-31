---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: same nodes, all of which need to apply simultaneously. Such constraints may even be in different files
---

same nodes, all of which need to apply simultaneously.
Such constraints may even be in different files.
But they may never contradict each other:
if one declaration says a field is 5, another may not override it to be 6.
Declaring a field to be both >5 and <10 is valid, though.

This approach is more restricted than full-blown inheritance;
it may not be possible to reuse existing configurations.
On the other hand, it is also a more powerful boilerplate remover.
For instance, suppose each job in a set needs to use a specific
