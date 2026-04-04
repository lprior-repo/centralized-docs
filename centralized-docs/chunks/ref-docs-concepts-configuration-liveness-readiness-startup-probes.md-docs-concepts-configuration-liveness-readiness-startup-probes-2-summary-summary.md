---
doc_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes
chunk_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 104
summary: * [Startup probe](#startup-probe) * [Liveness probe](#liveness-probe) * [Readiness probe](#readiness-probe)## Startup probe Startup probes verify whether the application within a container is...
---

* [Startup probe](#startup-probe)
* [Liveness probe](#liveness-probe)
* [Readiness probe](#readiness-probe)## Startup probe
Startup probes verify whether the application within a container is started. If a startup probe is configured,
Kubernetes does not execute liveness or readiness probes until the startup probe succeeds, allowing the application time to finish its initialization.
This type of probe is only executed at startup, unlike liveness and readiness probes, which are run periodically.