---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Safe and Unsafe Sysctls
token_count: 869
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
## Safe and Unsafe Sysctls
Kubernetes classes sysctls as either *safe* or *unsafe*. In addition to proper
namespacing, a *safe* sysctl must be properly *isolated* between pods on the
same node. This means that setting a *safe* sysctl for one pod
* must not have any influence on any other pod on the node
* must not allow to harm the node's health
* must not allow to gain CPU or memory resources outside of the resource limits
of a pod.
By far, most of the *namespaced* sysctls are not necessarily considered *safe*.
The following sysctls are supported in the *safe* set:
* `kernel.shm\_rmid\_forced`;
* `net.ipv4.ip\_local\_port\_range`;
* `net.ipv4.tcp\_syncookies`;
* `net.ipv4.ping\_group\_range` (since Kubernetes 1.18);
* `net.ipv4.ip\_unprivileged\_port\_start` (since Kubernetes 1.22);
* `net.ipv4.ip\_local\_reserved\_ports` (since Kubernetes 1.27, needs kernel 3.16+);
* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_fin\_timeout` (since Kubernetes 1.29, needs kernel 4.6+);
* `net.ipv4.tcp\_keepalive\_intvl` (since Kubernetes 1.29, needs kernel 4.5+);
* `net.ipv4.tcp\_keepalive\_probes` (since Kubernetes 1.29, needs kernel 4.5+).
* `net.ipv4.tcp\_rmem` (since Kubernetes 1.32, needs kernel 4.15+).
* `net.ipv4.tcp\_wmem` (since Kubernetes 1.32, needs kernel 4.15+).
#### Note:
There are some exceptions to the set of safe sysctls:
* The `net.\*` sysctls are not allowed with host networking enabled.
* The `net.ipv4.tcp\_syncookies` sysctl is not namespaced on Linux kernel version 4.5 or lower.
This list will be extended in future Kubernetes versions when the kubelet
supports better isolation mechanisms.
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