---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#19-summary
chunk_level: summary
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 107
summary: ``` `kubelet --allowed-unsafe-sysctls \\ 'kernel.msg\*,net.core.somaxconn' ... ` ``` For [Minikube](/docs/tasks/tools/#minikube), this can be done via the `extra-config` flag: ``` `minikube start...
---

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