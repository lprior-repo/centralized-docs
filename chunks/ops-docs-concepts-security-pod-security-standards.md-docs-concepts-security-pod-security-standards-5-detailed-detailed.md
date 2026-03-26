---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#5-detailed
chunk_level: detailed
chunk_type: table
heading: Table of Contents
token_count: 901
summary: * `spec.initContainers[\*].lifecycle.preStop.httpGet.host` **Allowed Values** * Undefined/nil * \"\"| |AppArmor| On supported hosts, the `RuntimeDefault` AppArmor profile is applied by default. The...
---

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
* `localhost/\*`|
|SELinux|
Setting the SELinux type is restricted, and setting a custom SELinux user or role option is forbidden.
**Restricted Fields**
* `spec.securityContext.seLinuxOptions.type`
* `spec.containers[\*].securityContext.seLinuxOptions.type`
* `spec.initContainers[\*].securityContext.seLinuxOptions.type`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.type`
**Allowed Values**
* Undefined/""
* `container\_t`
* `container\_init\_t`
* `container\_kvm\_t`
* `container\_engine\_t` (since Kubernetes 1.31)
**Restricted Fields**
* `spec.securityContext.seLinuxOptions.user`
* `spec.containers[\*].securityContext.seLinuxOptions.user`
* `spec.initContainers[\*].securityContext.seLinuxOptions.user`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.user`
* `spec.securityContext.seLinuxOptions.role`
* `spec.containers[\*].securityContext.seLinuxOptions.role`
* `spec.initContainers[\*].securityContext.seLinuxOptions.role`
* `spec.ephemeralContainers[\*].securityContext.seLinuxOptions.role`
**Allowed Values**
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