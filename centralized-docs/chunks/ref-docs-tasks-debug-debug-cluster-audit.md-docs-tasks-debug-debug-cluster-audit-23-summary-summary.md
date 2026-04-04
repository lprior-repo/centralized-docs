---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#23-summary
chunk_level: summary
chunk_type: prose
heading: Audit policy
token_count: 122
summary: If you're crafting your own audit profile, you can use the audit profile for Google Container-Optimized OS as a starting point. You can check the...
---

If you're crafting your own audit profile, you can use the audit profile for Google Container-Optimized OS as a starting point. You can check the
[configure-helper.sh](https://github.com/kubernetes/kubernetes/blob/master/cluster/gce/gci/configure-helper.sh)
script, which generates an audit policy file. You can see most of the audit policy file by looking directly at the script.
You can also refer to the [`Policy` configuration reference](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy)
for details about the fields defined.