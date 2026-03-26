---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#82-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 121
summary: `apiVersion: v1 kind: Secret metadata: # A bootstrap token Secret usually resides in the kube-system namespace namespace: kube-system type: bootstrap.kubernetes.io/token stringData:...
---

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