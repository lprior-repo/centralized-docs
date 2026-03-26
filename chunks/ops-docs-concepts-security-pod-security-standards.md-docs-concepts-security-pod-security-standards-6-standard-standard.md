---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#6-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 512
summary: * Undefined/nil * \"\"| |AppArmor| On supported hosts, the `RuntimeDefault` AppArmor profile is applied by default. The baseline policy should prevent overriding or disabling the default AppArmor...
---

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