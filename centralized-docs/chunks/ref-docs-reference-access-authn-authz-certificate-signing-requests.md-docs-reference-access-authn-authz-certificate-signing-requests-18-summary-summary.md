---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#18-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 126
summary: * Verbs: `approve`, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*` For example:...
---

* Verbs: `approve`, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
For example:
[`access/certificate-signing-request/clusterrole-approve.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/certificate-signing-request/clusterrole-approve.yaml)![](/images/copycode.svg "Copy access/certificate-signing-request/clusterrole-approve.yaml to clipboard")