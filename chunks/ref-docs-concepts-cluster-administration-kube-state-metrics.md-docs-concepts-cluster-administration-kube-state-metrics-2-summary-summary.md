---
doc_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics
chunk_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: kube-state-metrics, an add-on agent to generate and expose cluster-level metrics. The state of Kubernetes objects in the Kubernetes API can be exposed as metrics. An add-on agent called...
---

kube-state-metrics, an add-on agent to generate and expose cluster-level metrics.
The state of Kubernetes objects in the Kubernetes API can be exposed as metrics.
An add-on agent called [kube-state-metrics](https://github.com/kubernetes/kube-state-metrics) can connect to the Kubernetes API server and expose a HTTP endpoint with metrics generated from the state of individual objects in the cluster.
It exposes various information about the state of objects like labels and annotations, startup and termination times, status or the phase the object currently is in.
For example, containers running in pods create a `kube\_pod\_container\_info` metric.