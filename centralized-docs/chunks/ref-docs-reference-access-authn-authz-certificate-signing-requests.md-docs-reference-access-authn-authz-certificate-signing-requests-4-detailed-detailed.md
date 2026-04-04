---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Certificate signing requests
token_count: 670
summary: To allow approving a CertificateSigningRequest: * Verbs: `get`, `list`, `watch`, group: `certificates.k8s.io`, resource: `certificatesigningrequests` * Verbs: `update`, group: `certificates.k8s.io`,...
---

To allow approving a CertificateSigningRequest:
* Verbs: `get`, `list`, `watch`, group: `certificates.k8s.io`, resource: `certificatesigningrequests`
* Verbs: `update`, group: `certificates.k8s.io`, resource: `certificatesigningrequests/approval`
* Verbs: `approve`, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
For example:
[`access/certificate-signing-request/clusterrole-approve.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/certificate-signing-request/clusterrole-approve.yaml)![](/images/copycode.svg "Copy access/certificate-signing-request/clusterrole-approve.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: csr-approver
rules:
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests
verbs:
- get
- list
- watch
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests/approval
verbs:
- update
- apiGroups:
- certificates.k8s.io
resources:
- signers
resourceNames:
- example.com/my-signer-name # example.com/\* can be used to authorize for all signers in the 'example.com' domain
verbs:
- approve
`
```
To allow signing a CertificateSigningRequest:
* Verbs: `get`, `list`, `watch`, group: `certificates.k8s.io`, resource: `certificatesigningrequests`
* Verbs: `update`, group: `certificates.k8s.io`, resource: `certificatesigningrequests/status`
* Verbs: `sign`, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
[`access/certificate-signing-request/clusterrole-sign.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/certificate-signing-request/clusterrole-sign.yaml)![](/images/copycode.svg "Copy access/certificate-signing-request/clusterrole-sign.yaml to clipboard")
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: csr-signer
rules:
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests
verbs:
- get
- list
- watch
- apiGroups:
- certificates.k8s.io
resources:
- certificatesigningrequests/status
verbs:
- update
- apiGroups:
- certificates.k8s.io
resources:
- signers
resourceNames:
- example.com/my-signer-name # example.com/\* can be used to authorize for all signers in the 'example.com' domain
verbs:
- sign
`
```