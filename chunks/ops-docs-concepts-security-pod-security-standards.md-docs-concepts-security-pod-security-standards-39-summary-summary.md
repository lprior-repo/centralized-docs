---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#39-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 121
summary: * `spec.ephemeralContainers[\*].securityContext.runAsUser` **Allowed Values** * any non-zero value * `undefined/null`| |Seccomp (v1.19+)| Seccomp profile must be explicitly set to one of the allowed...
---

* `spec.ephemeralContainers[\*].securityContext.runAsUser`
**Allowed Values**
* any non-zero value
* `undefined/null`|
|Seccomp (v1.19+)|
Seccomp profile must be explicitly set to one of the allowed values. Both the `Unconfined` profile and the *absence* of a profile are prohibited. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(spec.os.name != windows)`*
**Restricted Fields**
* `spec.securityContext.seccompProfile.type`