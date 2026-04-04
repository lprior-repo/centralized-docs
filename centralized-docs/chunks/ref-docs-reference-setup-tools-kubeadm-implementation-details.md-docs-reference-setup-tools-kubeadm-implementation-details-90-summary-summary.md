---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#90-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 115
summary: * Then the CA certificate goes through following validation steps: * Basic validation: using the token ID against a JWT signature * Pub key validation: using provided...
---

* Then the CA certificate goes through following validation steps:
* Basic validation: using the token ID against a JWT signature
* Pub key validation: using provided `--discovery-token-ca-cert-hash`. This value is available
in the output of `kubeadm init` or can be calculated using standard tools (the hash is
calculated over the bytes of the Subject Public Key Info (SPKI) object as in RFC7469). The
`--discovery-token-ca-cert-hash flag` may be repeated multiple times to allow more than one public key.