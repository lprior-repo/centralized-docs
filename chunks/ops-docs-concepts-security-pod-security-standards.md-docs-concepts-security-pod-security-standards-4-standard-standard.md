---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#4-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 500
summary: * `spec.securityContext.windowsOptions.hostProcess` * `spec.containers[\*].securityContext.windowsOptions.hostProcess` * `spec.initContainers[\*].securityContext.windowsOptions.hostProcess` *...
---

* `spec.securityContext.windowsOptions.hostProcess`
* `spec.containers[\*].securityContext.windowsOptions.hostProcess`
* `spec.initContainers[\*].securityContext.windowsOptions.hostProcess`
* `spec.ephemeralContainers[\*].securityContext.windowsOptions.hostProcess`
**Allowed Values**
* Undefined/nil
* `false`|
|Host Namespaces|
Sharing the host namespaces must be disallowed.
**Restricted Fields**
* `spec.hostNetwork`
* `spec.hostPID`
* `spec.hostIPC`
**Allowed Values**
* Undefined/nil
* `false`|
|Privileged Containers|
Privileged Pods disable most security mechanisms and must be disallowed.
**Restricted Fields**
* `spec.containers[\*].securityContext.privileged`
* `spec.initContainers[\*].securityContext.privileged`
* `spec.ephemeralContainers[\*].securityContext.privileged`
**Allowed Values**
* Undefined/nil
* `false`|
|Capabilities|
Adding additional capabilities beyond those listed below must be disallowed.
**Restricted Fields**
* `spec.containers[\*].securityContext.capabilities.add`
* `spec.initContainers[\*].securityContext.capabilities.add`
* `spec.ephemeralContainers[\*].securityContext.capabilities.add`
**Allowed Values**
* Undefined/nil
* `AUDIT\_WRITE`
* `CHOWN`
* `DAC\_OVERRIDE`
* `FOWNER`
* `FSETID`
* `KILL`
* `MKNOD`
* `NET\_BIND\_SERVICE`
* `SETFCAP`
* `SETGID`
* `SETPCAP`
* `SETUID`
* `SYS\_CHROOT`|
|HostPath Volumes|
HostPath volumes must be forbidden.
**Restricted Fields**
* `spec.volumes[\*].hostPath`
**Allowed Values**
* Undefined/nil|
|Host Ports|
HostPorts should be disallowed entirely (recommended) or restricted to a known list
**Restricted Fields**
* `spec.containers[\*].ports[\*].hostPort`
* `spec.initContainers[\*].ports[\*].hostPort`
* `spec.ephemeralContainers[\*].ports[\*].hostPort`
**Allowed Values**
* Undefined/nil
* Known list (not supported by the built-in [Pod Security Admission controller](/docs/concepts/security/pod-security-admission/))