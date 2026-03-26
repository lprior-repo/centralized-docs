---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#8-summary
chunk_level: summary
chunk_type: prose
heading: Understanding the certificate rotation configuration
token_count: 54
summary: request and attach a signed certificate to the certificate signing request. The kubelet will retrieve the new signed certificate from the Kubernetes API and write that to disk. Then it will update...
---

request and attach a signed certificate to the certificate signing request. The
kubelet will retrieve the new signed certificate from the Kubernetes API and
write that to disk. Then it will update the connections it has to the
Kubernetes API to reconnect using the new certificate.