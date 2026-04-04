---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#13-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 217
summary: ### Custom signers You can also introduce your own custom signer, which should have a similar prefixed name but using your own domain name. For example, if you represent an open source project that...
---

### Custom signers
You can also introduce your own custom signer, which should have a similar prefixed name but using your
own domain name. For example, if you represent an open source project that uses the domain `open-fictional.example`
then you might use `issuer.open-fictional.example/service-mesh` as a signer name.
A custom signer uses the Kubernetes API to issue a certificate. See [API-based signers](#signer-api).
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