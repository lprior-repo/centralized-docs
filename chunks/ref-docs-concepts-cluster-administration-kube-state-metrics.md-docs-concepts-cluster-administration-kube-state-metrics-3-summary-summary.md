---
doc_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics
chunk_id: ref/docs-concepts-cluster-administration-kube-state-metrics.md/docs-concepts-cluster-administration-kube-state-metrics#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: For example, containers running in pods create a `kube\_pod\_container\_info` metric. This includes the name of the container, the name of the pod it is part of, the...
---

For example, containers running in pods create a `kube\_pod\_container\_info` metric.
This includes the name of the container, the name of the pod it is part of, the [namespace](/docs/concepts/overview/working-with-objects/namespaces) the pod is running in, the name of the container image, the ID of the image, the image name from the spec of the container, the ID of the running container and the ID of the pod as labels.
🛇 This item links to a third party project or product that is not part of Kubernetes itself.