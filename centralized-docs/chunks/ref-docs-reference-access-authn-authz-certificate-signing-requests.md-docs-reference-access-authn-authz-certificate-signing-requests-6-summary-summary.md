---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 105
summary: # Certificates and Certificate Signing Requests Kubernetes certificate and trust bundle APIs enable automation of [X.509](https://www.itu.int/rec/T-REC-X.509) credential provisioning by providing a...
---

# Certificates and Certificate Signing Requests
Kubernetes certificate and trust bundle APIs enable automation of
[X.509](https://www.itu.int/rec/T-REC-X.509) credential provisioning by providing
a programmatic interface for clients of the Kubernetes API to request and obtain
X.509 [certificates](/docs/tasks/tls/managing-tls-in-a-cluster/) from a Certificate Authority (CA).
There is also experimental (alpha) support for distributing [trust bundles](#cluster-trust-bundles).