---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#21-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 137
summary: added to the repository name.  That is, all repositories in the registry
---

added to the repository name. That is, all repositories in the registry
will be of the form repoPrefix/modulePath.

If there’s a +insecure suffix it specifies that an insecure HTTP
connection should be used to this registry. The default is to use a
secure HTTPS connection except for localhost addresses. For symmetry,
it’s also possible to use +secure to force an HTTPS connection even
on localhost connections.

For example, given:


Copy code
Copied!

CUE_REGISTRY=public-registry.example,github.com/acmecorp=registry.acme.example:6000/modules
