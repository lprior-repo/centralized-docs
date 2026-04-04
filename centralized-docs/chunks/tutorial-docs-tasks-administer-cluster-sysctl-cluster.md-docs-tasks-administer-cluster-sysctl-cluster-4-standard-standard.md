---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#4-standard
chunk_level: standard
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 232
summary: ### Enabling Unsafe Sysctls All *safe* sysctls are enabled by default. All *unsafe* sysctls are disabled by default and must be allowed manually by the cluster admin on a per-node basis. Pods with...
---

### Enabling Unsafe Sysctls
All *safe* sysctls are enabled by default.
All *unsafe* sysctls are disabled by default and must be allowed manually by the
cluster admin on a per-node basis. Pods with disabled unsafe sysctls will be
scheduled, but will fail to launch.
With the warning above in mind, the cluster admin can allow certain *unsafe*
sysctls for very special situations such as high-performance or real-time
application tuning. *Unsafe* sysctls are enabled on a node-by-node basis with a
flag of the kubelet; for example:
```
`kubelet --allowed-unsafe-sysctls \\
'kernel.msg\*,net.core.somaxconn' ...
`
```
For [Minikube](/docs/tasks/tools/#minikube), this can be done via the `extra-config` flag:
```
`minikube start --extra-config="kubelet.allowed-unsafe-sysctls=kernel.msg\*,net.core.somaxconn"...
`
```
Only *namespaced* sysctls can be enabled this way.