---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#48-summary
chunk_level: summary
chunk_type: prose
heading: Policy Instantiation
token_count: 121
summary: * [Kyverno](https://kyverno.io/policies/pod-security/) * [OPA Gatekeeper](https://github.com/open-policy-agent/gatekeeper)## Pod OS field Kubernetes lets you use nodes that run either Linux or...
---

* [Kyverno](https://kyverno.io/policies/pod-security/)
* [OPA Gatekeeper](https://github.com/open-policy-agent/gatekeeper)## Pod OS field
Kubernetes lets you use nodes that run either Linux or Windows. You can mix both kinds of
node in one cluster.
Windows in Kubernetes has some limitations and differentiators from Linux-based
workloads. Specifically, many of the Pod `securityContext` fields
[have no effect on Windows](/docs/concepts/windows/intro/#compatibility-v1-pod-spec-containers-securitycontext).