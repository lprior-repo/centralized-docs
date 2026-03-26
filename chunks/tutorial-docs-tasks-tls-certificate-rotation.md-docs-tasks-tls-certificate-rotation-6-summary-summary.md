---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#6-summary
chunk_level: summary
chunk_type: prose
heading: Understanding the certificate rotation configuration
token_count: 126
summary: Initially a certificate signing request from the kubelet on a node will have a status of `Pending`. If the certificate signing requests meets specific criteria, it will be auto approved by the...
---

Initially a certificate signing request from the kubelet on a node will have a
status of `Pending`. If the certificate signing requests meets specific
criteria, it will be auto approved by the controller manager, then it will have
a status of `Approved`. Next, the controller manager will sign a certificate,
issued for the duration specified by the
`--cluster-signing-duration` parameter, and the signed certificate
will be attached to the certificate signing request.
The kubelet will retrieve the signed certificate from the Kubernetes API and
write that to disk, in the location specified by `--cert-dir`. Then the kubelet