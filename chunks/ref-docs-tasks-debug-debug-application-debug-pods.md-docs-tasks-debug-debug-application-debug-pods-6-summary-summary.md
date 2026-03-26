---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#6-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 123
summary: ## Diagnosing the problem The first step in troubleshooting is triage. What is the problem? Is it your Pods, your Replication Controller or your Service? * [Debugging Pods](#debugging-pods) *...
---

## Diagnosing the problem
The first step in troubleshooting is triage. What is the problem?
Is it your Pods, your Replication Controller or your Service?
* [Debugging Pods](#debugging-pods)
* [Debugging Replication Controllers](#debugging-replication-controllers)
* [Debugging Services](#debugging-services)### Debugging Pods
The first step in debugging a Pod is taking a look at it. Check the current
state of the Pod and recent events with the following command:
```
`kubectl describe pods ${POD\_NAME}
`
```