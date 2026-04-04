---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#4-summary
chunk_level: summary
chunk_type: prose
heading: Transport security
token_count: 94
summary: to a generally recognized CA. The certificate and corresponding private key can be set by using the `--tls-cert-file` and `--tls-private-key-file` flags. If your cluster uses a private certificate...
---

to a generally recognized CA. The certificate and corresponding private key can be set
by using the `--tls-cert-file` and `--tls-private-key-file` flags.
If your cluster uses a private certificate authority, you need a copy of that CA
certificate configured into your `\~/.kube/config` on the client, so that you can
trust the connection and be confident it was not intercepted.
Your client can present a TLS client certificate at this stage.