---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#22-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 116
summary: * `spec.initContainers[\*].livenessProbe.tcpSocket.host` * `spec.initContainers[\*].readinessProbe.tcpSocket.host` * `spec.initContainers[\*].startupProbe.tcpSocket.host` *...
---

* `spec.initContainers[\*].livenessProbe.tcpSocket.host`
* `spec.initContainers[\*].readinessProbe.tcpSocket.host`
* `spec.initContainers[\*].startupProbe.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.postStart.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.preStop.tcpSocket.host`
* `spec.initContainers[\*].lifecycle.postStart.httpGet.host`
* `spec.initContainers[\*].lifecycle.preStop.httpGet.host`
**Allowed Values**
* Undefined/nil