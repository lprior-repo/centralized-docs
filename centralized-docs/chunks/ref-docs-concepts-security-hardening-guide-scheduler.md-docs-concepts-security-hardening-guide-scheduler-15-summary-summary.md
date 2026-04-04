---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#15-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 75
summary: * Plugins that implement the `permit` extension point can prevent or delay the binding of a Pod. Such plugins should be thoroughly reviewed by the cluster administrator. When using a plugin that is...
---

* Plugins that implement the `permit` extension point can prevent or delay the binding of a Pod.
Such plugins should be thoroughly reviewed by the cluster administrator.
When using a plugin that is not one of the [default plugins](/docs/reference/scheduling/config/#scheduling-plugins),
consider disabling the `queueSort`, `filter` and `permit` extension points as follows: