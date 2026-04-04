---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#75-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 119
summary: * `signerName`: The signer to which this request is addressed. * `podName` and `podUID`: The Pod that Kubelet is requesting a certificate for. * `serviceAccountName` and `serviceAccountUID`: The...
---

* `signerName`: The signer to which this request is addressed.
* `podName` and `podUID`: The Pod that Kubelet is requesting a certificate for.
* `serviceAccountName` and `serviceAccountUID`: The ServiceAccount corresponding to the Pod.
* `nodeName` and `nodeUID`: The Node corresponding to the Pod.
* `maxExpirationSeconds`: The maximum lifetime that the workload author will
accept for this certificate. Defaults to 24 hours if not specified.
* `pkixPublicKey`: The public key for which the certificate should be issued.