---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#12-standard
chunk_level: standard
chunk_type: prose
heading: Policy Instantiation
token_count: 446
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