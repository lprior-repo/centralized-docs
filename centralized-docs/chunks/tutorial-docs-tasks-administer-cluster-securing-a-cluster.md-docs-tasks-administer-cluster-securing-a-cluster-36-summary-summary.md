---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#36-summary
chunk_level: summary
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 83
summary: To block module loading more generically, you can use a Linux Security Module (such as SELinux) to completely deny the `module\_request` permission to containers, preventing the kernel from loading...
---

To block module loading more generically, you can use a Linux Security Module (such as
SELinux) to completely deny the `module\_request` permission to containers, preventing the
kernel from loading modules for containers under any circumstances. (Pods would still be
able to use modules that had been loaded manually, or modules that were loaded by the
kernel on behalf of some more-privileged process.)