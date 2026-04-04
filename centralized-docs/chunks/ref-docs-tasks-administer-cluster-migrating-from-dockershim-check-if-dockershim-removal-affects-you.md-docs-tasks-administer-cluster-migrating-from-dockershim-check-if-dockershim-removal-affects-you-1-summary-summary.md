---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 121
summary: # Check whether dockershim removal affects you The `dockershim` component of Kubernetes allows the use of Docker as a Kubernetes's [container...
---

# Check whether dockershim removal affects you
The `dockershim` component of Kubernetes allows the use of Docker as a Kubernetes's
[container runtime](/docs/setup/production-environment/container-runtimes).
Kubernetes' built-in `dockershim` component was removed in release v1.24.
This page explains how your cluster could be using Docker as a container runtime,
provides details on the role that `dockershim` plays when in use, and shows steps
you can take to check whether any workloads could be affected by `dockershim` removal.