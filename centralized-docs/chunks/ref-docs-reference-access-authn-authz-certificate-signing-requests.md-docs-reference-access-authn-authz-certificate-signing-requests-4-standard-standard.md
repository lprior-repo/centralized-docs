---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#4-standard
chunk_level: standard
chunk_type: prose
heading: Certificate signing requests
token_count: 436
summary: * Approved requests: automatically deleted after 1 hour * Denied requests: automatically deleted after 1 hour * Failed requests: automatically deleted after 1 hour * Pending requests: automatically...
---

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