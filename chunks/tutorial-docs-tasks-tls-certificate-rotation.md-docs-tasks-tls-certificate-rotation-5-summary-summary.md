---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#5-summary
chunk_level: summary
chunk_type: prose
heading: Understanding the certificate rotation configuration
token_count: 74
summary: ## Understanding the certificate rotation configuration When a kubelet starts up, if it is configured to bootstrap (using the `--bootstrap-kubeconfig` flag), it will use its initial certificate to...
---

## Understanding the certificate rotation configuration
When a kubelet starts up, if it is configured to bootstrap (using the
`--bootstrap-kubeconfig` flag), it will use its initial certificate to connect
to the Kubernetes API and issue a certificate signing request. You can view the
status of certificate signing requests using:
```
`kubectl get csr
`
```