---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 107
summary: * The kubelet's [Summary API](/docs/reference/config-api/kubelet-stats.v1alpha1/), which provides PSI data at the node, pod, and container level. * The `/metrics/cadvisor` endpoint on the kubelet,...
---

* The kubelet's [Summary API](/docs/reference/config-api/kubelet-stats.v1alpha1/), which provides PSI data at the node, pod, and container level.
* The `/metrics/cadvisor` endpoint on the kubelet, which exposes PSI metrics in the [Prometheus format](/docs/concepts/cluster-administration/system-metrics/#psi-metrics).### Requirements
Pressure Stall Information requires the following on your Linux nodes:
* The Linux kernel must be version **4.20 or newer**.