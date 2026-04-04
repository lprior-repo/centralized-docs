---
doc_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods
chunk_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 55
summary: ### Marking pod as critical To mark a Pod as critical, set priorityClassName for that Pod to `system-cluster-critical` or `system-node-critical`. `system-node-critical` is the highest available...
---

### Marking pod as critical
To mark a Pod as critical, set priorityClassName for that Pod to `system-cluster-critical` or `system-node-critical`. `system-node-critical` is the highest available priority, even higher than `system-cluster-critical`.