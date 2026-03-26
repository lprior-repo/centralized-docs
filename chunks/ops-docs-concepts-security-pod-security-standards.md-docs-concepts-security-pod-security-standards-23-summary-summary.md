---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#23-summary
chunk_level: summary
chunk_type: table
heading: Table of Contents
token_count: 117
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