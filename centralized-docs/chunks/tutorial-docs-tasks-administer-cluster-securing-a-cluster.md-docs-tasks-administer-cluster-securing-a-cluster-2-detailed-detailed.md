---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Controlling the capabilities of a workload or user at runtime
token_count: 1006
summary: ## Controlling the capabilities of a workload or user at runtime Authorization in Kubernetes is intentionally high level, focused on coarse actions on resources. More powerful controls exist as...
---

## Controlling the capabilities of a workload or user at runtime
Authorization in Kubernetes is intentionally high level, focused on coarse actions on resources.
More powerful controls exist as **policies** to limit by use case how those objects act on the
cluster, themselves, and other resources.
### Limiting resource usage on a cluster
[Resource quota](/docs/concepts/policy/resource-quotas/) limits the number or capacity of
resources granted to a namespace. This is most often used to limit the amount of CPU, memory,
or persistent disk a namespace can allocate, but can also control how many pods, services, or
volumes exist in each namespace.
[Limit ranges](/docs/tasks/administer-cluster/manage-resources/memory-default-namespace/) restrict the maximum or minimum size of some of the
resources above, to prevent users from requesting unreasonably high or low values for commonly
reserved resources like memory, or to provide default limits when none are specified.
### Controlling what privileges containers run with
A pod definition contains a [security context](/docs/tasks/configure-pod-container/security-context/)
that allows it to request access to run as a specific Linux user on a node (like root),
access to run privileged or access the host network, and other controls that would otherwise
allow it to run unfettered on a hosting node.
You can configure [Pod security admission](/docs/concepts/security/pod-security-admission/)
to enforce use of a particular [Pod Security Standard](/docs/concepts/security/pod-security-standards/)
in a [namespace](/docs/concepts/overview/working-with-objects/namespaces), or to detect breaches.
Generally, most application workloads need limited access to host resources so they can
successfully run as a root process (uid 0) without access to host information. However,
considering the privileges associated with the root user, you should write application
containers to run as a non-root user. Similarly, administrators who wish to prevent
client applications from escaping their containers should apply the **Baseline**
or **Restricted** Pod Security Standard.
### Preventing containers from loading unwanted kernel modules
The Linux kernel automatically loads kernel modules from disk if needed in certain
circumstances, such as when a piece of hardware is attached or a filesystem is mounted. Of
particular relevance to Kubernetes, even unprivileged processes can cause certain
network-protocol-related kernel modules to be loaded, just by creating a socket of the
appropriate type. This may allow an attacker to exploit a security hole in a kernel module
that the administrator assumed was not in use.
To prevent specific modules from being automatically loaded, you can uninstall them from
the node, or add rules to block them. On most Linux distributions, you can do that by
creating a file such as `/etc/modprobe.d/kubernetes-blacklist.conf` with contents like:
```
`# DCCP is unlikely to be needed, has had multiple serious
# SCTP is not used in most Kubernetes clusters, and has also had
# vulnerabilities in the past.
blacklist sctp
`
```
To block module loading more generically, you can use a Linux Security Module (such as
SELinux) to completely deny the `module\_request` permission to containers, preventing the
kernel from loading modules for containers under any circumstances. (Pods would still be
able to use modules that had been loaded manually, or modules that were loaded by the
kernel on behalf of some more-privileged process.)
### Restricting network access
The [network policies](/docs/tasks/administer-cluster/declare-network-policy/) for a namespace
allows application authors to restrict which pods in other namespaces may access pods and ports
within their namespaces. Many of the supported [Kubernetes networking providers](/docs/concepts/cluster-administration/networking/)
now respect network policy.
Quota and limit ranges can also be used to control whether users may request node ports or
load-balanced services, which on many clusters can control whether those users applications
are visible outside of the cluster.
Additional protections may be available that control network rules on a per-plugin or per-
environment basis, such as per-node firewalls, physically separating cluster nodes to
prevent cross talk, or advanced networking policy.
### Restricting cloud metadata API access
Cloud platforms (AWS, Azure, GCE, etc.) often expose metadata services locally to instances.
By default these APIs are accessible by pods running on an instance and can contain cloud
credentials for that node, or provisioning data such as kubelet credentials. These credentials
can be used to escalate within the cluster or to other cloud services under the same account.
When running Kubernetes on a cloud platform, limit permissions given to instance credentials, use
[network policies](/docs/tasks/administer-cluster/declare-network-policy/) to restrict pod access
to the metadata API, and avoid using provisioning data to deliver secrets.