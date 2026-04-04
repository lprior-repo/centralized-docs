---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#17-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 84
summary: ``` `--controllers=\*,bootstrapsigner ` ``` The ConfigMap that is signed is `cluster-info` in the `kube-public` namespace. The typical flow is that a client reads this ConfigMap while unauthenticated...
---

```
`--controllers=\*,bootstrapsigner
`
```
The ConfigMap that is signed is `cluster-info` in the `kube-public` namespace.
The typical flow is that a client reads this ConfigMap while unauthenticated and
ignoring TLS errors. It then validates the payload of the ConfigMap by looking
at a signature embedded in the ConfigMap.
The ConfigMap may look like this: