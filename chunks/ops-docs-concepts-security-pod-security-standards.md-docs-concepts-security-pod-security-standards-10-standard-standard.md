---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#10-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 467
summary: * `spec.volumes[\*]` **Allowed Values** Every item in the `spec.volumes[\*]` list must set one of the following fields to a non-null value: * `spec.volumes[\*].configMap` * `spec.volumes[\*].csi` *...
---

* `spec.volumes[\*]`
**Allowed Values**
Every item in the `spec.volumes[\*]` list must set one of the following fields to a non-null value:
* `spec.volumes[\*].configMap`
* `spec.volumes[\*].csi`
* `spec.volumes[\*].downwardAPI`
* `spec.volumes[\*].emptyDir`
* `spec.volumes[\*].ephemeral`
* `spec.volumes[\*].persistentVolumeClaim`
* `spec.volumes[\*].projected`
* `spec.volumes[\*].secret`|
|Privilege Escalation (v1.8+)|
Privilege escalation (such as via set-user-ID or set-group-ID file mode) should not be allowed. *[This is Linux only policy](#os-specific-policy-controls) in v1.25+ `(spec.os.name != windows)`*
**Restricted Fields**
* `spec.containers[\*].securityContext.allowPrivilegeEscalation`
* `spec.initContainers[\*].securityContext.allowPrivilegeEscalation`
* `spec.ephemeralContainers[\*].securityContext.allowPrivilegeEscalation`
**Allowed Values**
* `false`|
|Running as Non-root|
Containers must be required to run as non-root users.
**Restricted Fields**
* `spec.securityContext.runAsNonRoot`
* `spec.containers[\*].securityContext.runAsNonRoot`
* `spec.initContainers[\*].securityContext.runAsNonRoot`
* `spec.ephemeralContainers[\*].securityContext.runAsNonRoot`
**Allowed Values**
* `true`The container fields may be undefined/`nil` if the pod-level
`spec.securityContext.runAsNonRoot` is set to `true`.|
|Running as Non-root user (v1.23+)|
Containers must not set runAsUser to 0
**Restricted Fields**
* `spec.securityContext.runAsUser`
* `spec.containers[\*].securityContext.runAsUser`
* `spec.initContainers[\*].securityContext.runAsUser`
* `spec.ephemeralContainers[\*].securityContext.runAsUser`
**Allowed Values**
* any non-zero value