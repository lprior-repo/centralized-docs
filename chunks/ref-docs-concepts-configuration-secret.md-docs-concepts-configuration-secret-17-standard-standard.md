---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#17-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 410
summary: A bootstrap token Secret has the following keys specified under `data`: * `token-id`: A random 6 character string as the token identifier. Required. * `token-secret`: A random 16 character string as...
---

A bootstrap token Secret has the following keys specified under `data`:
* `token-id`: A random 6 character string as the token identifier. Required.
* `token-secret`: A random 16 character string as the actual token Secret. Required.
* `description`: A human-readable string that describes what the token is
used for. Optional.
* `expiration`: An absolute UTC time using [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) specifying when the token
should be expired. Optional.
* `usage-bootstrap-&lt;usage&gt;`: A boolean flag indicating additional usage for
the bootstrap token.
* `auth-extra-groups`: A comma-separated list of group names that will be
authenticated as in addition to the `system:bootstrappers` group.
You can alternatively provide the values in the `stringData` field of the Secret
without base64 encoding them:
[`secret/bootstrap-token-secret-literal.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/bootstrap-token-secret-literal.yaml)![](/images/copycode.svg "Copy secret/bootstrap-token-secret-literal.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
# A bootstrap token Secret usually resides in the kube-system namespace
namespace: kube-system
type: bootstrap.kubernetes.io/token
stringData:
auth-extra-groups: "system:bootstrappers:kubeadm:default-node-token"
expiration: "2020-09-13T04:39:10Z"
# This token ID is used in the name
token-id: "5emitj"
token-secret: "kq4gihvszzgn1p0r"
# This token can be used for authentication
usage-bootstrap-authentication: "true"
# and it can be used for signing
usage-bootstrap-signing: "true"`
```
#### Note:
The `stringData` field for a Secret does not work well with server-side apply.