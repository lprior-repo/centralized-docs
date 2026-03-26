---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#28-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 102
summary: ## What's next If none of the above solves your problem, follow the instructions in [Debugging Service document](/docs/tasks/debug/debug-application/debug-service/) to make sure that your `Service`...
---

## What's next
If none of the above solves your problem, follow the instructions in
[Debugging Service document](/docs/tasks/debug/debug-application/debug-service/)
to make sure that your `Service` is running, has `Endpoints`, and your `Pods` are
actually serving; you have DNS working, iptables rules installed, and kube-proxy
does not seem to be misbehaving.
You may also visit [troubleshooting document](/docs/tasks/debug/) for more information.