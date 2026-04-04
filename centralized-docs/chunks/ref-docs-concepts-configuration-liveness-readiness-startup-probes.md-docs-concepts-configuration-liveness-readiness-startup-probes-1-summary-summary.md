---
doc_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes
chunk_id: ref/docs-concepts-configuration-liveness-readiness-startup-probes.md/docs-concepts-configuration-liveness-readiness-startup-probes#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 66
summary: # Liveness, Readiness, and Startup Probes Kubernetes lets you define *probes* to continuously monitor the health of containers in a Pod. Based on probe results, Kubernetes can restart unhealthy...
---

# Liveness, Readiness, and Startup Probes
Kubernetes lets you define *probes* to continuously monitor the health of containers in a Pod.
Based on probe results, Kubernetes can restart unhealthy containers or stop sending traffic to containers that are not ready.
There are three types of probes, each serving a different purpose: