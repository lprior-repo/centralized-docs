---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 88
summary: ### Scheduler authentication &amp; authorization command line options When setting up authentication configuration, it should be made sure that kube-scheduler's authentication remains consistent with...
---

### Scheduler authentication &amp; authorization command line options
When setting up authentication configuration, it should be made sure that
kube-scheduler's authentication remains consistent with kube-api-server's authentication.
If any request has missing authentication headers, the authentication should happen through the kube-api-server
[allowing all authentication to be consistent in the cluster](/docs/tasks/extend-kubernetes/configure-aggregation-layer/#original-request-username-and-group).