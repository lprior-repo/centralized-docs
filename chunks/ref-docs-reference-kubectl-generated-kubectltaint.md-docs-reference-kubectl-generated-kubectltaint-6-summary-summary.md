---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#6-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 124
summary: * A taint consists of a key, value, and effect. As an argument here, it is expressed as key=value:effect. * The key must begin with a letter or number, and may contain letters, numbers, hyphens,...
---

* A taint consists of a key, value, and effect. As an argument here, it is expressed as key=value:effect.
* The key must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 253 characters.
* Optionally, the key can begin with a DNS subdomain prefix and a single '/', like example.com/my-app.
* The value is optional. If given, it must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters.