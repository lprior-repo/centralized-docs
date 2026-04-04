---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#6-summary
chunk_level: summary
chunk_type: prose
heading: TokenRequestSpec
token_count: 88
summary: * **audiences** ([]string), required *Atomic: will be replaced during a merge* Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in...
---

* **audiences** ([]string), required
*Atomic: will be replaced during a merge*
Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.