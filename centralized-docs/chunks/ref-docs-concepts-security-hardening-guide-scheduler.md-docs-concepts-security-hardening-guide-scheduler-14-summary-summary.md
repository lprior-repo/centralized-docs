---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#14-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 70
summary: * Exactly one plugin that uses the `queueSort` extension point can be enabled at a time. Any plugins that use `queueSort` should be scrutinized. * Plugins that implement the `prefilter` or `filter`...
---

* Exactly one plugin that uses the `queueSort` extension point can be enabled at a time.
Any plugins that use `queueSort` should be scrutinized.
* Plugins that implement the `prefilter` or `filter` extension point can potentially mark all nodes as unschedulable.
This can bring scheduling of new pods to a halt.