---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: there is any ambiguity with respect to regular module dependencies an. “ambiguous import” error will be reported
---

there is any ambiguity with respect to regular module dependencies an
“ambiguous import” error will be reported.

MODULES, PACKAGES, AND VERSIONS

A module is a collection of packages that are released,
versioned, and distributed together. Modules are downloaded from
OCI-compliant [https://github.com/opencontainers/distribution-spec/blob/main/spec.md]
artifact registries. This means that if you are deploying CUE to the cloud,
you can use the same distribution mechanism that you might be using for
Docker images to deploy your CUE configuration too.
