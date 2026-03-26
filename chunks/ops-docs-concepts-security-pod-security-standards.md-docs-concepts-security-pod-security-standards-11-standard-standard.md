---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#11-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 476
summary: * `spec.securityContext.runAsUser` * `spec.containers[\*].securityContext.runAsUser` * `spec.initContainers[\*].securityContext.runAsUser` * `spec.ephemeralContainers[\*].securityContext.runAsUser`...
---

* `spec.securityContext.runAsUser`
* `spec.containers[\*].securityContext.runAsUser`
* `spec.initContainers[\*].securityContext.runAsUser`
* `spec.ephemeralContainers[\*].securityContext.runAsUser`
**Allowed Values**
* any non-zero value
* `undefined/null`|
|Seccomp (v1.19+)|
Seccomp profile must be explicitly set to one of the allowed values. Both the `Unconfined` profile and the *absence* of a profile are prohibited. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(spec.os.name != windows)`*
**Restricted Fields**
* `spec.securityContext.seccompProfile.type`
* `spec.containers[\*].securityContext.seccompProfile.type`
* `spec.initContainers[\*].securityContext.seccompProfile.type`
* `spec.ephemeralContainers[\*].securityContext.seccompProfile.type`
**Allowed Values**
* `RuntimeDefault`
* `Localhost`The container fields may be undefined/`nil` if the pod-level
`spec.securityContext.seccompProfile.type` field is set appropriately.
Conversely, the pod-level field may be undefined/`nil` if \_all\_ container-
level fields are set.|
|Capabilities (v1.22+)|
Containers must drop `ALL` capabilities, and are only permitted to add back
the `NET\_BIND\_SERVICE` capability. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(.spec.os.name != "windows")`*
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.drop`
* `spec.initContainers[\*].securityContext.capabilities.drop`
* `spec.ephemeralContainers[\*].securityContext.capabilities.drop`
**Allowed Values**
* Any list of capabilities that includes `ALL`
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.add`
* `spec.initContainers[\*].securityContext.capabilities.add`
* `spec.ephemeralContainers[\*].securityContext.capabilities.add`
**Allowed Values**
* Undefined/nil
* `NET\_BIND\_SERVICE`|