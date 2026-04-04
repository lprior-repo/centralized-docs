---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 610
summary: ## Table of Contents  - [Auditing](#auditing)       - [Note:](#note)   - [Audit policy](#audit-policy) - [Don't generate audit events for all requests in RequestReceived...
---

## Table of Contents

- [Auditing](#auditing)
      - [Note:](#note)
  - [Audit policy](#audit-policy)
- [Don't generate audit events for all requests in RequestReceived stage.](#dont-generate-audit-events-for-all-requests-in-requestreceived-stage)
- [Log pod changes at RequestResponse level](#log-pod-changes-at-requestresponse-level)
- [Resource "pods" doesn't match requests to any subresource of pods,](#resource-pods-doesnt-match-requests-to-any-subresource-of-pods)
- [which is consistent with the RBAC policy.](#which-is-consistent-with-the-rbac-policy)
- [Log "pods/log", "pods/status" at Metadata level](#log-podslog-podsstatus-at-metadata-level)
- [Don't log requests to a configmap called "controller-leader"](#dont-log-requests-to-a-configmap-called-controller-leader)
- [Don't log watch requests by the "system:kube-proxy" on endpoints or services](#dont-log-watch-requests-by-the-systemkube-proxy-on-endpoints-or-services)
- [Don't log authenticated requests to certain non-resource URL paths.](#dont-log-authenticated-requests-to-certain-non-resource-url-paths)
- [Log the request body of configmap changes in kube-system.](#log-the-request-body-of-configmap-changes-in-kube-system)
- [This rule only applies to resources in the "kube-system" namespace.](#this-rule-only-applies-to-resources-in-the-kube-system-namespace)
- [The empty string "" can be used to select non-namespaced resources.](#the-empty-string--can-be-used-to-select-non-namespaced-resources)
- [Log configmap and secret changes in all other namespaces at the Metadata level.](#log-configmap-and-secret-changes-in-all-other-namespaces-at-the-metadata-level)
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