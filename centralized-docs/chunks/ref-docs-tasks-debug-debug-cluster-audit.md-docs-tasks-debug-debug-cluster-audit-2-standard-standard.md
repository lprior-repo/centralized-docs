---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 205
summary: - [Log all other resources in core and extensions at the Request level.](#log-all-other-resources-in-core-and-extensions-at-the-request-level) - [A catch-all rule to log all other requests at the...
---

- [Log all other resources in core and extensions at the Request level.](#log-all-other-resources-in-core-and-extensions-at-the-request-level)
- [A catch-all rule to log all other requests at the Metadata level.](#a-catch-all-rule-to-log-all-other-requests-at-the-metadata-level)
- [Long-running requests like watches that fall under this rule will not](#long-running-requests-like-watches-that-fall-under-this-rule-will-not)
- [generate an audit event in RequestReceived.](#generate-an-audit-event-in-requestreceived)
  - [Audit backends](#audit-backends)
      - [Note:](#note)
    - [Log backend](#log-backend)
    - [Webhook backend](#webhook-backend)
  - [Event batching](#event-batching)
  - [Parameter tuning](#parameter-tuning)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---