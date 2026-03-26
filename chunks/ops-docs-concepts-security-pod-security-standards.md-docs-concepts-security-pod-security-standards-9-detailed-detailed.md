---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 677
summary: ### What's the difference between a security profile and a security context? [Security Contexts](/docs/tasks/configure-pod-container/security-context/) configure Pods and Containers at runtime....
---

### What's the difference between a security profile and a security context?
[Security Contexts](/docs/tasks/configure-pod-container/security-context/) configure Pods and
Containers at runtime. Security contexts are defined as part of the Pod and container specifications
in the Pod manifest, and represent parameters to the container runtime.
Security profiles are control plane mechanisms to enforce specific settings in the Security Context,
as well as other related parameters outside the Security Context. As of July 2021,
[Pod Security Policies](/docs/concepts/security/pod-security-policy/) are deprecated in favor of the
built-in [Pod Security Admission Controller](/docs/concepts/security/pod-security-admission/).
### What about sandboxed Pods?
There is currently no API standard that controls whether a Pod is considered sandboxed or
not. Sandbox Pods may be identified by the use of a sandboxed runtime (such as gVisor or Kata
Containers), but there is no standard definition of what a sandboxed runtime is.
The protections necessary for sandboxed workloads can differ from others. For example, the need to
restrict privileged permissions is lessened when the workload is isolated from the underlying
kernel. This allows for workloads requiring heightened permissions to still be isolated.
Additionally, the protection of sandboxed workloads is highly dependent on the method of
sandboxing. As such, no single recommended profile is recommended for all sandboxed workloads.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified August 06, 2025 at 6:48 PM PST: [nit-fix: Add empty value for host field in probes PSA (a0fb9cc6b3)](https://github.com/kubernetes/website/commit/a0fb9cc6b3bdc96b6df50a6ab6778140150ea484)
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects. See the [CNCF website guidelines](https://github.com/cncf/foundation/blob/main/policies-guidance/website-guidelines.md) for more details.
You should read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before proposing a change that adds an extra third-party link.
## Related Pages

- [Other Tools](docs-reference-tools.md)
- [Metrics for Kubernetes Object States](docs-concepts-cluster-administration-kube-state-metrics.md)
- [Developing and debugging services locally using telepresence](docs-tasks-debug-debug-cluster-local-debugging.md)
- [Service Accounts](docs-concepts-security-service-accounts.md)
- [pod security policy](docs-concepts-security-pod-security-policy.md)