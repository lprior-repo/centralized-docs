---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#6-summary
chunk_level: summary
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 122
summary: is software that can execute the containers that make up a Kubernetes pod. Kubernetes is responsible for orchestration and scheduling of Pods; on each node, the...
---

 is software that can
execute the containers that make up a Kubernetes pod. Kubernetes is responsible for orchestration
and scheduling of Pods; on each node, the [kubelet](/docs/reference/command-line-tools-reference/kubelet)
uses the container runtime interface as an abstraction so that you can use any compatible
container runtime.
In its earliest releases, Kubernetes offered compatibility with one container runtime: Docker.
Later in the Kubernetes project's history, cluster operators wanted to adopt additional container runtimes.
The CRI was designed to allow this kind of flexibility - and the kubelet began supporting CRI. However,