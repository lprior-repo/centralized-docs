---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#19-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: This creates a scheduler profile `my-scheduler`. Whenever the `.spec` of a Pod does not have a value for `.spec.schedulerName`, the kube-scheduler runs for that Pod, using its main configuration, and...
---

This creates a scheduler profile `my-scheduler`.
Whenever the `.spec` of a Pod does not have a value for `.spec.schedulerName`, the kube-scheduler runs for that Pod,
using its main configuration, and default plugins.
If you define a Pod with `.spec.schedulerName` set to `my-scheduler`, the kube-scheduler runs
but with a custom configuration; in that custom configuration,
the `queueSort`, `filter` and `permit` extension points are disabled.
If you use this KubeSchedulerConfiguration, and don't run any custom scheduler,
and you then define a Pod with `.spec.schedulerName`