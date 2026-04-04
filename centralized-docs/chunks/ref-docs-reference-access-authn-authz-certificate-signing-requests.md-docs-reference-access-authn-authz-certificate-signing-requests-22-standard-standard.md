---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#22-standard
chunk_level: standard
chunk_type: prose
heading: Approval or rejection
token_count: 502
summary: * `signerName`: The signer to which this request is addressed. * `podName` and `podUID`: The Pod that Kubelet is requesting a certificate for. * `serviceAccountName` and `serviceAccountUID`: The...
---

* `signerName`: The signer to which this request is addressed.
* `podName` and `podUID`: The Pod that Kubelet is requesting a certificate for.
* `serviceAccountName` and `serviceAccountUID`: The ServiceAccount corresponding to the Pod.
* `nodeName` and `nodeUID`: The Node corresponding to the Pod.
* `maxExpirationSeconds`: The maximum lifetime that the workload author will
accept for this certificate. Defaults to 24 hours if not specified.
* `pkixPublicKey`: The public key for which the certificate should be issued.
* `proofOfPossession`: A signature demonstrating that the requester controls the
private key corresponding to `pkixPublicKey`.
* `unverifiedUserAnnotations`: A map that allows the user to pass additional
information to the signer implementation. It is copied verbatim from the
`userAnnotations` field of the [podCertificate projected volume source](/docs/concepts/storage/projected-volumes/#podcertificate).
Entries are subject to the same validation as object metadata annotations,
with the addition that all keys must be domain-prefixed. No restrictions are
placed on values, except an overall size limitation on the entire field. Other
than these basic validations, the API server does not conduct any extra
validations. The signer implementations should be very careful when consuming
this data. Signers must not inherently trust this data without first
performing the appropriate verification steps. Signers should document the
keys and values they support. Signers should deny requests that contain keys
they do not recognize.
Nodes automatically receive permissions to create PodCertificateRequests and
read PodCertificateRequests related to them (as determined by the
`spec.nodeName` field). The `NodeRestriction` admission plugin, if enabled,
ensures that nodes can only create PodCertificateRequests that correspond to a
real pod that is currently running on the node.
After creation, the `spec` of a PodCertificateRequest is immutable.
Unlike CSRs, PodCertificateRequests do not have an
approval phase. Once the PodCertificateRequest is created, the signer's
controller directly decides to issue or deny the request. It also has the
option to mark the request as failed, if it encountered a permanent error when
attempting to issue the request.
To take any of these actions, the signing controller needs to have the
appropriate permissions on both the PodCertificateRequest type, as well as on
the signer name: