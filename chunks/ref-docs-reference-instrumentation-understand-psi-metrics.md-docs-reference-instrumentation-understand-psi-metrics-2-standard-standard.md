---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 428
summary: * The kubelet's [Summary API](/docs/reference/config-api/kubelet-stats.v1alpha1/), which provides PSI data at the node, pod, and container level. * The `/metrics/cadvisor` endpoint on the kubelet,...
---

* The kubelet's [Summary API](/docs/reference/config-api/kubelet-stats.v1alpha1/), which provides PSI data at the node, pod, and container level.
* The `/metrics/cadvisor` endpoint on the kubelet, which exposes PSI metrics in the [Prometheus format](/docs/concepts/cluster-administration/system-metrics/#psi-metrics).### Requirements
Pressure Stall Information requires the following on your Linux nodes:
* The Linux kernel must be version **4.20 or newer**.
* The kernel must be compiled with the `CONFIG\_PSI=y` option. Most modern distributions enable this by default. You can check your kernel's configuration by running `zgrep CONFIG\_PSI /proc/config.gz`.
* Some Linux distributions may compile PSI into the kernel but disable it by default. If so, you need to enable it at boot time by adding the `psi=1` parameter to the kernel command line.
* The node must be using [cgroup v2](/docs/concepts/architecture/cgroups/).## Understanding PSI Metrics
Pressure Stall Information (PSI) metrics are provided for three resources: CPU, memory, and I/O. They are categorized into two main types of pressure: `some` and `full`.
* **`some`**: This value indicates that some tasks (one or more) are stalled on a resource. For example, if some tasks are waiting for I/O, this metric will increase. This can be an early indicator of resource contention.
* **`full`**: This value indicates that *all* non-idle tasks are stalled on a resource simultaneously. This signifies a more severe resource shortage, where the entire system is unable to make progress.
Each pressure type provides four metrics: `avg10`, `avg60`, `avg300`, and `total`. The `avg` values represent the percentage of wall-clock time that tasks were stalled over 10-second, 60-second, and 5-minute moving averages. The `total` value is a cumulative counter in microseconds showing the total time tasks have been stalled.