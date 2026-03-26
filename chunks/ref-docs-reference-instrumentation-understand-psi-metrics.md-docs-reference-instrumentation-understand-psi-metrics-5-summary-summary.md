---
doc_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics
chunk_id: ref/docs-reference-instrumentation-understand-psi-metrics.md/docs-reference-instrumentation-understand-psi-metrics#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: Detailed explanation of Pressure Stall Information (PSI) metrics and how to use them to identify resource pressure in Kubernetes. FEATURE STATE: `Kubernetes v1.34 [beta]` As a beta feature,...
---

Detailed explanation of Pressure Stall Information (PSI) metrics and how to use them to identify resource pressure in Kubernetes.
FEATURE STATE:
`Kubernetes v1.34 [beta]`
As a beta feature, Kubernetes lets you configure the kubelet to collect Linux kernel
[Pressure Stall Information](https://docs.kernel.org/accounting/psi.html)
(PSI) for CPU, memory, and I/O usage. The information is collected at node, pod and container level.
This feature is enabled by default by setting the `KubeletPSI` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)