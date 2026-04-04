---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#50-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 125
summary: ### Control plane signer The Kubernetes control plane implements each of the [Kubernetes signers](/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers), as part of the...
---

### Control plane signer
The Kubernetes control plane implements each of the
[Kubernetes signers](/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers),
as part of the kube-controller-manager.
#### Note:
Prior to Kubernetes v1.18, the kube-controller-manager would sign any CSRs that
were marked as approved.
#### Note:
The `spec.expirationSeconds` field was added in Kubernetes v1.22.
Earlier versions of Kubernetes do not honor this field.
Kubernetes API servers prior to v1.22 will silently drop this field when the object is created.