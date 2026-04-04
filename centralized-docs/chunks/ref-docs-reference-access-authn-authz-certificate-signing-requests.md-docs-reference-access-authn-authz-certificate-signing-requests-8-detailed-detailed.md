---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Signers
token_count: 545
summary: 1. Trust distribution: None. There is no standard trust or distribution for this signer in a Kubernetes cluster. 2. Permitted subjects - any 3. Permitted x509 extensions - honors subjectAltName and...
---

1. Trust distribution: None. There is no standard trust or distribution for this signer in a Kubernetes cluster.
2. Permitted subjects - any
3. Permitted x509 extensions - honors subjectAltName and key usage extensions and discards other extensions.
4. Permitted key usages - any
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.
The kube-controller-manager implements [control plane signing](#signer-control-plane) for each of the built in
signers. Failures for all of these are only reported in kube-controller-manager logs.
#### Note:
The `spec.expirationSeconds` field was added in Kubernetes v1.22. Earlier versions of Kubernetes do not honor this field.
Kubernetes API servers prior to v1.22 will silently drop this field when the object is created.
Distribution of trust happens out of band for these signers. Any trust outside of those described above are strictly
coincidental. For instance, some distributions may honor `kubernetes.io/legacy-unknown` as client certificates for the
kube-apiserver, but this is not a standard.
None of these usages are related to ServiceAccount token secrets `.data[ca.crt]` in any way. That CA bundle is only
guaranteed to verify a connection to the API server using the default service (`kubernetes.default.svc`).
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