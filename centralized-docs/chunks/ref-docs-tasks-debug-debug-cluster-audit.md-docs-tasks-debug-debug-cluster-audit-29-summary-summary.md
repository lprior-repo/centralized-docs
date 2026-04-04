---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#29-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 119
summary: then mount the volumes: ``` `... volumeMounts: - mountPath: /etc/kubernetes/audit-policy.yaml name: audit readOnly: true - mountPath: /var/log/kubernetes/audit/ name: audit-log readOnly: false ` ```...
---

then mount the volumes:
```
`...
volumeMounts:
- mountPath: /etc/kubernetes/audit-policy.yaml
name: audit
readOnly: true
- mountPath: /var/log/kubernetes/audit/
name: audit-log
readOnly: false
`
```
and finally configure the `hostPath`:
```
`...
volumes:
- name: audit
hostPath:
path: /etc/kubernetes/audit-policy.yaml
type: File
- name: audit-log
hostPath:
path: /var/log/kubernetes/audit/
type: DirectoryOrCreate
`
```