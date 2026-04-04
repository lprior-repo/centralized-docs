---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#48-summary
chunk_level: summary
chunk_type: prose
heading: Protecting cluster components from compromise
token_count: 124
summary: ### Enable audit logging The [audit logger](/docs/tasks/debug/debug-cluster/audit/) is a beta feature that records actions taken by the API for later analysis in the event of a compromise. It is...
---

### Enable audit logging
The [audit logger](/docs/tasks/debug/debug-cluster/audit/) is a beta feature that records actions taken by the
API for later analysis in the event of a compromise. It is recommended to enable audit logging
and archive the audit file on a secure server.
### Restrict access to alpha or beta features
Alpha and beta Kubernetes features are in active development and may have limitations or bugs
that result in security vulnerabilities. Always assess the value an alpha or beta feature may
provide against the possible risk to your security posture. When in doubt, disable features you
do not use.