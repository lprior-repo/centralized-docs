---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: * `profiling`: Set to `false` to disable the profiling endpoints which are provide debugging information but which should not be enabled on production clusters as they present a risk of denial of...
---

* `profiling`: Set to `false` to disable the profiling endpoints which are provide debugging information
but which should not be enabled on production clusters as they present a risk of denial of service
or information leakage. The `--profiling` argument is deprecated and can now be provided through the
[KubeScheduler DebuggingConfiguration](/docs/reference/config-api/kube-scheduler-config.v1/#DebuggingConfiguration).
Profiling can be disabled through the kube-scheduler config by setting `enableProfiling` to `false`.
* `requestheader-client-ca-file`: Avoid passing this argument.### Scheduler networking command line options