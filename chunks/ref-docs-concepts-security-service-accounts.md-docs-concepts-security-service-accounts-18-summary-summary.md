---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#18-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 72
summary: 3. Assign the ServiceAccount object to Pods during Pod creation. If you're using the identity from an external service, [retrieve the ServiceAccount token](#get-a-token) and use it from that service...
---

3. Assign the ServiceAccount object to Pods during Pod creation.
If you're using the identity from an external service,
[retrieve the ServiceAccount token](#get-a-token) and use it from that
service instead.
For instructions, refer to
[Configure Service Accounts for Pods](/docs/tasks/configure-pod-container/configure-service-account/).