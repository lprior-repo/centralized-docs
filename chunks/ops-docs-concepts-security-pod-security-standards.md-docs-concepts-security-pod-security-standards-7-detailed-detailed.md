---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#7-detailed
chunk_level: detailed
chunk_type: table
heading: Table of Contents
token_count: 986
summary: #### Note: In this table, wildcards (`\*`) indicate all elements in a list. For example, `spec.containers[\*].securityContext` refers to the Security Context object for *all defined containers*. If...
---

#### Note:
In this table, wildcards (`\*`) indicate all elements in a list. For example,
`spec.containers[\*].securityContext` refers to the Security Context object for *all defined
containers*. If any of the listed containers fails to meet the requirements, the entire pod will
fail validation.
Restricted policy specification|**Control**|**Policy**|
|*Everything from the Baseline policy*|
|Volume Types|
The Restricted policy only permits the following volume types.
**Restricted Fields**
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