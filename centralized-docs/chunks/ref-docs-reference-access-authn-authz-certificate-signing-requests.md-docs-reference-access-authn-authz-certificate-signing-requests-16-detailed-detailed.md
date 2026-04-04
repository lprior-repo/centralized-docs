---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#16-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 861
summary: ### Signer-unlinked ClusterTrustBundles Signer-unlinked ClusterTrustBundles have an empty `spec.signerName` field, like this: ``` `apiVersion: certificates.k8s.io/v1alpha1 kind: ClusterTrustBundle...
---

### Signer-unlinked ClusterTrustBundles
Signer-unlinked ClusterTrustBundles have an empty `spec.signerName` field, like this:
```
`apiVersion: certificates.k8s.io/v1alpha1
kind: ClusterTrustBundle
metadata:
name: foo
spec:
# no signerName specified, so the field is blank
trustBundle: "&lt;... PEM data ...&gt;"
`
```
They are primarily intended for cluster configuration use cases.
Each signer-unlinked ClusterTrustBundle is an independent object, in contrast to the
customary grouping behavior of signer-linked ClusterTrustBundles.
Signer-unlinked ClusterTrustBundles have no `attest` verb requirement.
Instead, you control access to them directly using the usual mechanisms,
such as role-based access control.
To distinguish them from signer-linked ClusterTrustBundles, the names of
signer-unlinked ClusterTrustBundles **must not** contain a colon (`:`).
### Accessing ClusterTrustBundles from pods
FEATURE STATE:
`Kubernetes v1.33 [beta]`(disabled by default)
The contents of ClusterTrustBundles can be injected into the container filesystem, similar to ConfigMaps and Secrets.
See the [clusterTrustBundle projected volume source](/docs/concepts/storage/projected-volumes/#clustertrustbundle) for more details.
## What's next
* Read [Manage TLS Certificates in a Cluster](/docs/tasks/tls/managing-tls-in-a-cluster/)
* Read [Issue a Certificate for a Kubernetes API Client Using A CertificateSigningRequest](/docs/tasks/tls/certificate-issue-client-csr/)
* View the source code for the kube-controller-manager built in
[signer](https://github.com/kubernetes/kubernetes/blob/32ec6c212ec9415f604ffc1f4c1f29b782968ff1/pkg/controller/certificates/signer/cfssl_signer.go)
* View the source code for the kube-controller-manager built in
[approver](https://github.com/kubernetes/kubernetes/blob/32ec6c212ec9415f604ffc1f4c1f29b782968ff1/pkg/controller/certificates/approver/sarapprove.go)
* For details of X.509 itself, refer to [RFC 5280](https://tools.ietf.org/html/rfc5280#section-3.1) section 3.1
* For information on the syntax of PKCS#10 certificate signing requests, refer to [RFC 2986](https://tools.ietf.org/html/rfc2986)
* Read about the ClusterTrustBundle API:
* %!s()
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified November 20, 2025 at 5:02 AM PST: [KEP-4317: Docs Signed-off-by: Monis Khan &lt;mok@microsoft.com&gt; (05dbf3b839)](https://github.com/kubernetes/website/commit/05dbf3b839e807c56a1bb42adb94198a3fe2ae6f)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [Use an Image Volume With a Pod](docs-tasks-configure-pod-container-image-volumes.md)
- [Controlling Access to the Kubernetes API](docs-concepts-security-controlling-access.md)