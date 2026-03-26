---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#2-standard
chunk_level: standard
chunk_type: prose
heading: Full metrics pipeline
token_count: 494
summary: ## Full metrics pipeline A full metrics pipeline gives you access to richer metrics. Kubernetes can respond to these metrics by automatically scaling or adapting the cluster based on its current...
---

## Full metrics pipeline
A full metrics pipeline gives you access to richer metrics. Kubernetes can
respond to these metrics by automatically scaling or adapting the cluster
based on its current state, using mechanisms such as the Horizontal Pod
Autoscaler. The monitoring pipeline fetches metrics from the kubelet and
then exposes them to Kubernetes via an adapter by implementing either the
`custom.metrics.k8s.io` or `external.metrics.k8s.io` API.
Kubernetes is designed to work with [OpenMetrics](https://openmetrics.io/),
which is one of the
[CNCF Observability and Analysis - Monitoring Projects](https://landscape.cncf.io/?group=projects-and-products&amp;view-mode=card#observability-and-analysis--monitoring),
built upon and carefully extending [Prometheus exposition format](https://prometheus.io/docs/instrumenting/exposition_formats/)
in almost 100% backwards-compatible ways.
If you glance over at the
[CNCF Landscape](https://landscape.cncf.io/?group=projects-and-products&amp;view-mode=card#observability-and-analysis--monitoring),
you can see a number of monitoring projects that can work with Kubernetes by *scraping*
metric data and using that to help you observe your cluster. It is up to you to select the tool
or tools that suit your needs. The CNCF landscape for observability and analytics includes a
mix of open-source software, paid-for software-as-a-service, and other commercial products.
When you design and implement a full metrics pipeline you can make that monitoring data
available back to Kubernetes. For example, a HorizontalPodAutoscaler can use the processed
metrics to work out how many Pods to run for a component of your workload.
Integration of a full metrics pipeline into your Kubernetes implementation is outside
the scope of Kubernetes documentation because of the very wide scope of possible
solutions.
The choice of monitoring platform depends heavily on your needs, budget, and technical resources.
Kubernetes does not recommend any specific metrics pipeline; [many options](https://landscape.cncf.io/?group=projects-and-products&amp;view-mode=card#observability-and-analysis--monitoring) are available.
Your monitoring system should be capable of handling the [OpenMetrics](https://openmetrics.io/) metrics
transmission standard and needs to be chosen to best fit into your overall design and deployment of
your infrastructure platform.