---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Certificate signing requests
token_count: 890
summary: ### Request signing process The CertificateSigningRequest resource type allows a client to ask for an X.509 certificate be issued, based on a signing request. The CertificateSigningRequest object...
---

### Request signing process
The CertificateSigningRequest resource type allows a client to ask for an X.509 certificate
be issued, based on a signing request.
The CertificateSigningRequest object includes a PEM-encoded PKCS#10 signing request in
the `spec.request` field. The CertificateSigningRequest denotes the signer (the
recipient that the request is being made to) using the `spec.signerName` field.
Note that `spec.signerName` is a required key after API version `certificates.k8s.io/v1`.
In Kubernetes v1.22 and later, clients may optionally set the `spec.expirationSeconds`
field to request a particular lifetime for the issued certificate. The minimum valid
value for this field is `600`, i.e. ten minutes.
Once created, a CertificateSigningRequest must be approved before it can be signed.
Depending on the signer selected, a CertificateSigningRequest may be automatically approved
by a [controller](/docs/concepts/architecture/controller/).
Otherwise, a CertificateSigningRequest must be manually approved either via the REST API (or client-go)
or by running `kubectl certificate approve`. Likewise, a CertificateSigningRequest may also be denied,
which tells the configured signer that it must not sign the request.
For certificates that have been approved, the next step is signing. The relevant signing controller
first validates that the signing conditions are met and then creates a certificate.
The signing controller then updates the CertificateSigningRequest, storing the new certificate into
the `status.certificate` field of the existing CertificateSigningRequest object. The
`status.certificate` field is either empty or contains a X.509 certificate, encoded in PEM format.
The CertificateSigningRequest `status.certificate` field is empty until the signer does this.
Once the `status.certificate` field has been populated, the request has been completed and clients can now
fetch the signed certificate PEM data from the CertificateSigningRequest resource.
The signers can instead deny certificate signing if the approval conditions are not met.
In order to reduce the number of old CertificateSigningRequest resources left in a cluster, a garbage collection
controller runs periodically. The garbage collection removes CertificateSigningRequests that have not changed
state for some duration:
* Approved requests: automatically deleted after 1 hour
* Denied requests: automatically deleted after 1 hour
* Failed requests: automatically deleted after 1 hour
* Pending requests: automatically deleted after 24 hours
* All requests: automatically deleted after the issued certificate has expired### Certificate signing authorization
To allow creating a CertificateSigningRequest and retrieving any CertificateSigningRequest:
* Verbs: `create`, `get`, `list`, `watch`, group: `certificates.k8s.io`, resource: `certificatesigningrequests`
For example:
[`access/certificate-signing-request/clusterrole-create.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/certificate-signing-request/clusterrole-create.yaml)![](/images/copycode.svg "Copy access/certificate-signing-request/clusterrole-create.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: csr-creator
rules:
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests
verbs:
- create
- get
- list
- watch
`
```
To allow approving a CertificateSigningRequest:
* Verbs: `get`, `list`, `watch`, group: `certificates.k8s.io`, resource: `certificatesigningrequests`
* Verbs: `update`, group: `certificates.k8s.io`, resource: `certificatesigningrequests/approval`
* Verbs: `approve`, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
For example:
[`access/certificate-signing-request/clusterrole-approve.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/certificate-signing-request/clusterrole-approve.yaml)![](/images/copycode.svg "Copy access/certificate-signing-request/clusterrole-approve.yaml to clipboard")