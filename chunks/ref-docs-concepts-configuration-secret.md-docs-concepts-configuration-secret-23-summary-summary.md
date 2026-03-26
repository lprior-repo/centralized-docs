---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#23-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 40
summary: command: - ls - \"-l\" - \"/etc/secret-volume\" volumeMounts: - name: secret-volume readOnly: true mountPath: \"/etc/secret-volume\"` ```
---

command:
- ls
- "-l"
- "/etc/secret-volume"
volumeMounts:
- name: secret-volume
readOnly: true
mountPath: "/etc/secret-volume"`
```