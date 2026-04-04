---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#20-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 66
summary: kubeconfig: | apiVersion: v1 clusters: - cluster: certificate-authority-data: &lt;really long certificate data&gt; server: https://10.138.0.2:6443 name: \"\" contexts: [] current-context: \"\" kind:...
---

kubeconfig: |
apiVersion: v1
clusters:
- cluster:
certificate-authority-data: &lt;really long certificate data&gt;
server: https://10.138.0.2:6443
name: ""
contexts: []
current-context: ""
kind: Config
preferences: {}
users: []
`
```