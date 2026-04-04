---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 102
summary: * `requestheader-client-ca-file`: Avoid passing this argument.### Scheduler networking command line options * `bind-address`: In most cases, the kube-scheduler does not need to be externally...
---

* `requestheader-client-ca-file`: Avoid passing this argument.### Scheduler networking command line options
* `bind-address`: In most cases, the kube-scheduler does not need to be externally accessible.
Setting the bind address to `localhost` is a secure practice.
* `permit-address-sharing`: Set this to `false` to disable connection sharing through `SO\_REUSEADDR`.
`SO\_REUSEADDR` can lead to reuse of terminated connections that are in `TIME\_WAIT` state.