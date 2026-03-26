---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#19-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 121
summary: * `spec.ephemeralContainers[\*].ports[\*].hostPort` **Allowed Values** * Undefined/nil * Known list (not supported by the built-in [Pod Security Admission...
---

* `spec.ephemeralContainers[\*].ports[\*].hostPort`
**Allowed Values**
* Undefined/nil
* Known list (not supported by the built-in [Pod Security Admission controller](/docs/concepts/security/pod-security-admission/))
* `0`|
|Host Probes / Lifecycle Hooks (v1.34+)|
The Host field in probes and lifecycle hooks must be disallowed.
**Restricted Fields**
* `spec.containers[\*].livenessProbe.httpGet.host`
* `spec.containers[\*].readinessProbe.httpGet.host`