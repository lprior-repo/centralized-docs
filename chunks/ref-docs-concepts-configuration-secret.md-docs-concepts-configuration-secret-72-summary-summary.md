---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#72-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 68
summary: ``` `kubectl create secret tls my-tls-secret \\ --cert=path/to/cert/file \\ --key=path/to/key/file ` ``` The public/private key pair must exist before hand. The public key certificate for `--cert`...
---

```
`kubectl create secret tls my-tls-secret \\
--cert=path/to/cert/file \\
--key=path/to/key/file
`
```
The public/private key pair must exist before hand. The public key certificate for `--cert` must be .PEM encoded
and must match the given private key for `--key`.