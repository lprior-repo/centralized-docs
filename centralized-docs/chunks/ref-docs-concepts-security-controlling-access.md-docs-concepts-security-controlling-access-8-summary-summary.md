---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#8-summary
chunk_level: summary
chunk_type: prose
heading: Authentication
token_count: 54
summary: also provide the group memberships of the user, while other authenticators do not. While Kubernetes uses usernames for access control decisions and in request logging, it does not have a `User`...
---

also provide the group memberships of the user, while other authenticators
do not.
While Kubernetes uses usernames for access control decisions and in request logging,
it does not have a `User` object nor does it store usernames or other information about
users in its API.