---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#65-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 59
summary: #### Caution: SSH private keys do not establish trusted communication between an SSH client and host server on their own. A secondary means of establishing trust is needed to mitigate \"man in the...
---

#### Caution:
SSH private keys do not establish trusted communication between an SSH client and
host server on their own. A secondary means of establishing trust is needed to
mitigate "man in the middle" attacks, such as a `known\_hosts` file added to a ConfigMap.