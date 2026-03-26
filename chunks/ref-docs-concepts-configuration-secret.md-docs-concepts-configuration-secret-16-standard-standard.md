---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#16-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 339
summary: ### Bootstrap token Secrets The `bootstrap.kubernetes.io/token` Secret type is for tokens used during the node bootstrap process. It stores tokens used to sign well-known ConfigMaps. A bootstrap...
---

### Bootstrap token Secrets
The `bootstrap.kubernetes.io/token` Secret type is for
tokens used during the node bootstrap process. It stores tokens used to sign
well-known ConfigMaps.
A bootstrap token Secret is usually created in the `kube-system` namespace and
named in the form `bootstrap-token-&lt;token-id&gt;` where `&lt;token-id&gt;` is a 6 character
string of the token ID.
As a Kubernetes manifest, a bootstrap token Secret might look like the
following:
[`secret/bootstrap-token-secret-base64.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/bootstrap-token-secret-base64.yaml)![](/images/copycode.svg "Copy secret/bootstrap-token-secret-base64.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: bootstrap-token-5emitj
namespace: kube-system
type: bootstrap.kubernetes.io/token
data:
auth-extra-groups: c3lzdGVtOmJvb3RzdHJhcHBlcnM6a3ViZWFkbTpkZWZhdWx0LW5vZGUtdG9rZW4=
expiration: MjAyMC0wOS0xM1QwNDozOToxMFo=
token-id: NWVtaXRq
token-secret: a3E0Z2lodnN6emduMXAwcg==
usage-bootstrap-authentication: dHJ1ZQ==
usage-bootstrap-signing: dHJ1ZQ==`
```
A bootstrap token Secret has the following keys specified under `data`: