---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: Such requirements can be specified across files. This approach not only reduces the boilerplate contained in acmeMonitoring
---

Such requirements can be specified across files.

This approach not only reduces the boilerplate contained in acmeMonitoring
but also removes the repetitiveness of having to specify
this template for each job in jobs.
At the same time, this statement acts as a type enforcement.
This dual function is a key aspect of CUE and
typed feature structure languages in general.

This approach breaks down, of course, if the restrictions in
acmeMonitoring are too stringent and jobs need to override them.
To this extent, CUE provides mechanisms to allow defaults, opt-out, and
