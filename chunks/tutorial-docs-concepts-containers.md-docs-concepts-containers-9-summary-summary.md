---
doc_id: tutorial/docs-concepts-containers.md/docs-concepts-containers
chunk_id: tutorial/docs-concepts-containers.md/docs-concepts-containers#9-summary
chunk_level: summary
chunk_type: prose
heading: Container runtimes
token_count: 90
summary: . Usually, you can allow your cluster to pick the default container runtime for a Pod. If you need to use more than one container runtime in your cluster, you can specify the...
---

.
Usually, you can allow your cluster to pick the default container runtime
for a Pod. If you need to use more than one container runtime in your cluster,
you can specify the [RuntimeClass](/docs/concepts/containers/runtime-class/)
for a Pod to make sure that Kubernetes runs those containers using a
particular container runtime.
You can also use RuntimeClass to run different Pods with the same container
runtime but with different settings.