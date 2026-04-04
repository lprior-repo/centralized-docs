---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#59-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 121
summary: --tls-cert-file. | |--tls-sni-cert-key string| || A pair of x509 certificate and private key file paths, optionally suffixed with a list of domain patterns which are fully qualified domain names,...
---

--tls-cert-file.
|
|--tls-sni-cert-key string|
||
A pair of x509 certificate and private key file paths, optionally suffixed with a list of domain patterns which are fully qualified domain names, possibly with prefixed wildcard segments. The domain patterns also allow IP addresses, but IPs should only be used if the apiserver has visibility to the IP address requested by a client. If no domain patterns are provided, the names of the certificate are extracted. Non-wildcard matches trump over wildcard matches, explicit domain patterns trump over extracted names. For multiple key/certificate pairs, use the --