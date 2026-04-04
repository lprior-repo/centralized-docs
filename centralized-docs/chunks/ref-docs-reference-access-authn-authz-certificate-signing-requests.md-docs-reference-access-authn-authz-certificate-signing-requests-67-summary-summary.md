---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#67-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 95
summary: ### Approval or rejection using `kubectl` A Kubernetes administrator (with appropriate permissions) can manually approve (or deny) CertificateSigningRequests by using the `kubectl certificate...
---

### Approval or rejection using `kubectl`
A Kubernetes administrator (with appropriate permissions) can manually approve
(or deny) CertificateSigningRequests by using the `kubectl certificate approve` and `kubectl certificate deny` commands.
To approve a CSR with kubectl:
```
`kubectl certificate approve &lt;certificate-signing-request-name&gt;
`
```
Likewise, to deny a CSR:
```
`kubectl certificate deny &lt;certificate-signing-request-name&gt;
`
```