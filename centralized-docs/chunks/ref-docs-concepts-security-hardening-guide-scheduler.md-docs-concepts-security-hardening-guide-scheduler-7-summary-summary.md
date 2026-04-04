---
doc_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler
chunk_id: ref/docs-concepts-security-hardening-guide-scheduler.md/docs-concepts-security-hardening-guide-scheduler#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 111
summary: * `authentication-kubeconfig`: Make sure to provide a proper kubeconfig so that the scheduler can retrieve authentication configuration options from the API Server. This kubeconfig file should be...
---

* `authentication-kubeconfig`: Make sure to provide a proper kubeconfig so that
the scheduler can retrieve authentication configuration options from the API Server.
This kubeconfig file should be protected with strict file permissions.
* `authentication-tolerate-lookup-failure`: Set this to `false` to make sure
the scheduler *always* looks up its authentication configuration from the API server.
* `authentication-skip-lookup`: Set this to `false` to make sure
the scheduler *always* looks up its authentication configuration from the API server.