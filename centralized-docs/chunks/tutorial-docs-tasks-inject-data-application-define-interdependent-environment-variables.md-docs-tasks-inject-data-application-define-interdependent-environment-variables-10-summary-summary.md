---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 77
summary: 2. List the running Pods: ``` `kubectl get pods dependent-envars-demo ` ``` ``` `NAME READY STATUS RESTARTS AGE dependent-envars-demo 1/1 Running 0 9s ` ``` 3. Check the logs for the container...
---

2. List the running Pods:
```
`kubectl get pods dependent-envars-demo
`
```
```
`NAME READY STATUS RESTARTS AGE
dependent-envars-demo 1/1 Running 0 9s
`
```
3. Check the logs for the container running in your Pod:
```
`kubectl logs pod/dependent-envars-demo
`
```