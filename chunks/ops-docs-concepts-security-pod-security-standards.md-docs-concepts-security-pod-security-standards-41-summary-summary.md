---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#41-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 128
summary: * `Localhost`The container fields may be undefined/`nil` if the pod-level `spec.securityContext.seccompProfile.type` field is set appropriately. Conversely, the pod-level field may be undefined/`nil`...
---

* `Localhost`The container fields may be undefined/`nil` if the pod-level
`spec.securityContext.seccompProfile.type` field is set appropriately.
Conversely, the pod-level field may be undefined/`nil` if \_all\_ container-
level fields are set.|
|Capabilities (v1.22+)|
Containers must drop `ALL` capabilities, and are only permitted to add back
the `NET\_BIND\_SERVICE` capability. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(.spec.os.name != "windows")`*