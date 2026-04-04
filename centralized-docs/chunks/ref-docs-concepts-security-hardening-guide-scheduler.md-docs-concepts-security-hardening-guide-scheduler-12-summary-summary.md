---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#12-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 117
summary: * `tls-cipher-suites`: Always provide a list of preferred cipher suites. This ensures encryption never happens with insecure cipher suites.## Scheduling configurations for custom schedulers When...
---

* `tls-cipher-suites`: Always provide a list of preferred cipher suites.
This ensures encryption never happens with insecure cipher suites.## Scheduling configurations for custom schedulers
When using custom schedulers based on the Kubernetes scheduling code, cluster administrators need to be careful with
plugins that use the `queueSort`, `prefilter`, `filter`, or `permit` [extension points](/docs/reference/scheduling/config/#extension-points).
These extension points control various stages of a scheduling process,
and the wrong configuration can impact the kube-scheduler's behavior in your cluster.