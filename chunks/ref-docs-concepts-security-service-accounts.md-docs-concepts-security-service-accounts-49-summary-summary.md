---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#49-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating service account credentials
token_count: 63
summary: as valid until the token reaches its expiration timestamp. Your application should always define the audience that it accepts, and should check that the token's audiences match the audiences that the...
---

as valid until the token reaches its expiration timestamp.
Your application should always define the audience that it accepts, and should
check that the token's audiences match the audiences that the application
expects. This helps to minimize the scope of the token so that it can only be
used in your application and nowhere else.