---
doc_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards
chunk_id: ops/docs-concepts-security-pod-security-standards.md/docs-concepts-security-pod-security-standards#21-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: * `spec.containers[\*].lifecycle.preStop.tcpSocket.host` * `spec.containers[\*].lifecycle.postStart.httpGet.host` * `spec.containers[\*].lifecycle.preStop.httpGet.host` *...
---

* `spec.containers[\*].lifecycle.preStop.tcpSocket.host`
* `spec.containers[\*].lifecycle.postStart.httpGet.host`
* `spec.containers[\*].lifecycle.preStop.httpGet.host`
* `spec.initContainers[\*].livenessProbe.httpGet.host`
* `spec.initContainers[\*].readinessProbe.httpGet.host`
* `spec.initContainers[\*].startupProbe.httpGet.host`
* `spec.initContainers[\*].livenessProbe.tcpSocket.host`
* `spec.initContainers[\*].readinessProbe.tcpSocket.host`