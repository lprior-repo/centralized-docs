---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#28-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 128
summary: * `--audit-log-maxbackup` defines the maximum number of audit log files to retain * `--audit-log-maxsize` defines the maximum size in megabytes of the audit log file before it gets rotated If your...
---

* `--audit-log-maxbackup` defines the maximum number of audit log files to retain
* `--audit-log-maxsize` defines the maximum size in megabytes of the audit log file before it gets rotated
If your cluster's control plane runs the kube-apiserver as a Pod, remember to mount the `hostPath`
to the location of the policy file and log file, so that audit records are persisted. For example:
```
` - --audit-policy-file=/etc/kubernetes/audit-policy.yaml
- --audit-log-path=/var/log/kubernetes/audit/audit.log
`
```
then mount the volumes: