---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Policy Instantiation
token_count: 890
summary: ## Policy Instantiation Decoupling policy definition from policy instantiation allows for a common understanding and consistent language of policies across clusters, independent of the underlying...
---

## Policy Instantiation
Decoupling policy definition from policy instantiation allows for a common understanding and
consistent language of policies across clusters, independent of the underlying enforcement
mechanism.
As mechanisms mature, they will be defined below on a per-policy basis. The methods of enforcement
of individual policies are not defined here.
[**Pod Security Admission Controller**](/docs/concepts/security/pod-security-admission/)
* [Privileged namespace](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/security/podsecurity-privileged.yaml)
* [Baseline namespace](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/security/podsecurity-baseline.yaml)
* [Restricted namespace](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/security/podsecurity-restricted.yaml)### Alternatives
**Note:** This section links to third party projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for these projects, which are listed alphabetically. To add a project to this list, read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before submitting a change. [More information.](#third-party-content-disclaimer)
Other alternatives for enforcing policies are being developed in the Kubernetes ecosystem, such as:
* [Kubewarden](https://github.com/kubewarden)
* [Kyverno](https://kyverno.io/policies/pod-security/)
* [OPA Gatekeeper](https://github.com/open-policy-agent/gatekeeper)## Pod OS field
Kubernetes lets you use nodes that run either Linux or Windows. You can mix both kinds of
node in one cluster.
Windows in Kubernetes has some limitations and differentiators from Linux-based
workloads. Specifically, many of the Pod `securityContext` fields
[have no effect on Windows](/docs/concepts/windows/intro/#compatibility-v1-pod-spec-containers-securitycontext).
#### Note:
Kubelets prior to v1.24 don't enforce the pod OS field, and if a cluster has nodes on versions earlier than v1.24 the Restricted policies should be pinned to a version prior to v1.25.
### Restricted Pod Security Standard changes
Another important change, made in Kubernetes v1.25 is that the *Restricted* policy
has been updated to use the `pod.spec.os.name` field. Based on the OS name, certain policies that are specific
to a particular OS can be relaxed for the other OS.
#### OS-specific policy controls
Restrictions on the following controls are only required if `.spec.os.name` is not `windows`:
* Privilege Escalation
* Seccomp
* Linux Capabilities## User namespaces
User Namespaces are a Linux-only feature to run workloads with increased
isolation. How they work together with Pod Security Standards is described in
the [documentation](/docs/concepts/workloads/pods/user-namespaces/#integration-with-pod-security-admission-checks) for Pods that use user namespaces.
### Why isn't there a profile between Privileged and Baseline?
The three profiles defined here have a clear linear progression from most secure (Restricted) to least
secure (Privileged), and cover a broad set of workloads. Privileges required above the Baseline
policy are typically very application specific, so we do not offer a standard profile in this
niche. This is not to say that the privileged profile should always be used in this case, but that
policies in this space need to be defined on a case-by-case basis.
SIG Auth may reconsider this position in the future, should a clear need for other profiles arise.
### What's the difference between a security profile and a security context?
[Security Contexts](/docs/tasks/configure-pod-container/security-context/) configure Pods and
Containers at runtime. Security contexts are defined as part of the Pod and container specifications
in the Pod manifest, and represent parameters to the container runtime.
Security profiles are control plane mechanisms to enforce specific settings in the Security Context,
as well as other related parameters outside the Security Context. As of July 2021,
[Pod Security Policies](/docs/concepts/security/pod-security-policy/) are deprecated in favor of the
built-in [Pod Security Admission Controller](/docs/concepts/security/pod-security-admission/).