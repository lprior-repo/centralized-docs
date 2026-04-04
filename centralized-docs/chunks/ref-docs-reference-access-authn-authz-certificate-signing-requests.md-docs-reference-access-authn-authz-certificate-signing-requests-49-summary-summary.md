---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#49-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 92
summary: ### Custom signers You can also introduce your own custom signer, which should have a similar prefixed name but using your own domain name. For example, if you represent an open source project that...
---

### Custom signers
You can also introduce your own custom signer, which should have a similar prefixed name but using your
own domain name. For example, if you represent an open source project that uses the domain `open-fictional.example`
then you might use `issuer.open-fictional.example/service-mesh` as a signer name.
A custom signer uses the Kubernetes API to issue a certificate. See [API-based signers](#signer-api).