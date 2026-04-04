---
doc_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes
chunk_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 127
summary: * Read more about the [Configure Liveness, Readiness and Startup Probes](/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/).## Liveness probe Liveness probes determine...
---

* Read more about the [Configure Liveness, Readiness and Startup Probes](/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/).## Liveness probe
Liveness probes determine when to restart a container. For example, liveness probes could catch a deadlock when an application is running but unable to make progress.
If a container fails its liveness probe repeatedly, the kubelet restarts the container.
Liveness probes do not wait for readiness probes to succeed. If you want to wait before executing a liveness probe, you can either define `initialDelaySeconds` or use a