---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#5-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 511
summary: * `spec.containers[\*].ports[\*].hostPort` * `spec.initContainers[\*].ports[\*].hostPort` * `spec.ephemeralContainers[\*].ports[\*].hostPort` **Allowed Values** * Undefined/nil * Known list (not...
---

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