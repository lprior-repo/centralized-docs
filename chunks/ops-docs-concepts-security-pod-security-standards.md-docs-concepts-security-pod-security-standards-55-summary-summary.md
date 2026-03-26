---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#55-summary
chunk_level: summary
chunk_type: prose
heading: Policy Instantiation
token_count: 125
summary: [Security Contexts](/docs/tasks/configure-pod-container/security-context/) configure Pods and Containers at runtime. Security contexts are defined as part of the Pod and container specifications in...
---

[Security Contexts](/docs/tasks/configure-pod-container/security-context/) configure Pods and
Containers at runtime. Security contexts are defined as part of the Pod and container specifications
in the Pod manifest, and represent parameters to the container runtime.
Security profiles are control plane mechanisms to enforce specific settings in the Security Context,
as well as other related parameters outside the Security Context. As of July 2021,
[Pod Security Policies](/docs/concepts/security/pod-security-policy/) are deprecated in favor of the
built-in [Pod Security Admission Controller](/docs/concepts/security/pod-security-admission/).