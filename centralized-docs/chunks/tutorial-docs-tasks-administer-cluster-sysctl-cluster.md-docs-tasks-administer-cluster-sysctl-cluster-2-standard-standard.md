---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#2-standard
chunk_level: standard
chunk_type: prose
heading: Listing all Sysctl Parameters
token_count: 139
summary: ## Listing all Sysctl Parameters In Linux, the sysctl interface allows an administrator to modify kernel parameters at runtime. Parameters are available via the `/proc/sys/` virtual process file...
---

## Listing all Sysctl Parameters
In Linux, the sysctl interface allows an administrator to modify kernel
parameters at runtime. Parameters are available via the `/proc/sys/` virtual
process file system. The parameters cover various subsystems such as:
* kernel (common prefix: `kernel.`)
* networking (common prefix: `net.`)
* virtual memory (common prefix: `vm.`)
* MDADM (common prefix: `dev.`)
* More subsystems are described in [Kernel docs](https://www.kernel.org/doc/Documentation/sysctl/README).
To get a list of all parameters, you can run
```
`sudo sysctl -a
`
```