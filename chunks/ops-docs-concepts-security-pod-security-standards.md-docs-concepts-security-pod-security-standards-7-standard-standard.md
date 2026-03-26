---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#7-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 451
summary: * Undefined/\"\"| |`/proc` Mount Type| The default `/proc` masks are set up to reduce attack surface, and should be required. **Restricted Fields** * `spec.containers[\*].securityContext.procMount` *...
---

* Undefined/""|
|`/proc` Mount Type|
The default `/proc` masks are set up to reduce attack surface, and should be required.
**Restricted Fields**
* `spec.containers[\*].securityContext.procMount`
* `spec.initContainers[\*].securityContext.procMount`
* `spec.ephemeralContainers[\*].securityContext.procMount`
**Allowed Values**
* Undefined/nil
* `Default`|
|Seccomp|
Seccomp profile must not be explicitly set to `Unconfined`.
**Restricted Fields**
* `spec.securityContext.seccompProfile.type`
* `spec.containers[\*].securityContext.seccompProfile.type`
* `spec.initContainers[\*].securityContext.seccompProfile.type`
* `spec.ephemeralContainers[\*].securityContext.seccompProfile.type`
**Allowed Values**
* Undefined/nil
* `RuntimeDefault`
* `Localhost`|
|Sysctls|
Sysctls can disable security mechanisms or affect all containers on a host, and should be disallowed except for an allowed "safe" subset. A sysctl is considered safe if it is namespaced in the container or the Pod, and it is isolated from other Pods or processes on the same Node.
**Restricted Fields**
* `spec.securityContext.sysctls[\*].name`
**Allowed Values**
* Undefined/nil
* `kernel.shm\_rmid\_forced`
* `net.ipv4.ip\_local\_port\_range`
* `net.ipv4.ip\_unprivileged\_port\_start`
* `net.ipv4.tcp\_syncookies`
* `net.ipv4.ping\_group\_range`
* `net.ipv4.ip\_local\_reserved\_ports` (since Kubernetes 1.27)
* `net.ipv4.tcp\_keepalive\_time` (since Kubernetes 1.29)
* `net.ipv4.tcp\_fin\_timeout` (since Kubernetes 1.29)
* `net.ipv4.tcp\_keepalive\_intvl` (since Kubernetes 1.29)
* `net.ipv4.tcp\_keepalive\_probes` (since Kubernetes 1.29)|