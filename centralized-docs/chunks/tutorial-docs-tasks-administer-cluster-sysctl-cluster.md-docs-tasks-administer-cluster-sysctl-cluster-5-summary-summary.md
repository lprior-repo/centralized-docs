---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 117
summary: Starting from Kubernetes version 1.23, the kubelet supports the use of either `/` or `.` as separators for sysctl names. Starting from Kubernetes version 1.25, setting Sysctls for a Pod supports...
---

Starting from Kubernetes version 1.23, the kubelet supports the use of either `/` or `.`
as separators for sysctl names.
Starting from Kubernetes version 1.25, setting Sysctls for a Pod supports setting sysctls with slashes.
For example, you can represent the same sysctl name as `kernel.shm\_rmid\_forced` using a
period as the separator, or as `kernel/shm\_rmid\_forced` using a slash as a separator.
For more sysctl parameter conversion method details, please refer to
the page