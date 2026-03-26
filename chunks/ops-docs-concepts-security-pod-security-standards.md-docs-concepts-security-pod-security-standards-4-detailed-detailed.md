---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#4-detailed
chunk_level: detailed
chunk_type: table
heading: Table of Contents
token_count: 1009
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
* `0`|
|Host Probes / Lifecycle Hooks (v1.34+)|
The Host field in probes and lifecycle hooks must be disallowed.
**Restricted Fields**
* `spec.containers[\*].livenessProbe.httpGet.host`
* `spec.containers[\*].readinessProbe.httpGet.host`
* `spec.containers[\*].startupProbe.httpGet.host`
* `spec.containers[\*].livenessProbe.tcpSocket.host`
* `spec.containers[\*].readinessProbe.tcpSocket.host`
* `spec.containers[\*].startupProbe.tcpSocket.host`
* `spec.containers[\*].lifecycle.postStart.tcpSocket.host`
* `spec.containers[\*].lifecycle.preStop.tcpSocket.host`
* `spec.containers[\*].lifecycle.postStart.httpGet.host`
* `spec.containers[\*].lifecycle.preStop.httpGet.host`
* `spec.initContainers[\*].livenessProbe.httpGet.host`
* `spec.initContainers[\*].readinessProbe.httpGet.host`
* `spec.initContainers[\*].startupProbe.httpGet.host`
* `spec.initContainers[\*].livenessProbe.tcpSocket.host`
* `spec.initContainers[\*].readinessProbe.tcpSocket.host`
* `spec.initContainers[\*].startupProbe.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.postStart.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.preStop.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.postStart.httpGet.host`
* `spec.initContainers[\*].lifecycle.preStop.httpGet.host`
**Allowed Values**
* Undefined/nil
* ""|
|AppArmor|
On supported hosts, the `RuntimeDefault` AppArmor profile is applied by default. The baseline policy should prevent overriding or disabling the default AppArmor profile, or restrict overrides to an allowed set of profiles.
**Restricted Fields**
* `spec.securityContext.appArmorProfile.type`
* `spec.containers[\*].securityContext.appArmorProfile.type`
* `spec.initContainers[\*].securityContext.appArmorProfile.type`
* `spec.ephemeralContainers[\*].securityContext.appArmorProfile.type`
**Allowed Values**
* Undefined/nil
* `RuntimeDefault`
* `Localhost`
* `metadata.annotations["container.apparmor.security.beta.kubernetes.io/\*"]`
**Allowed Values**
* Undefined/nil
* `runtime/default`