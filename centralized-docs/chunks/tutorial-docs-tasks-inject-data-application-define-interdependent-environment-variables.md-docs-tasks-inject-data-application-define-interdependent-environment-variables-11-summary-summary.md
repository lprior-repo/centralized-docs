---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 87
summary: 3. Check the logs for the container running in your Pod: ``` `kubectl logs pod/dependent-envars-demo ` ``` ``` ` UNCHANGED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80...
---

3. Check the logs for the container running in your Pod:
```
`kubectl logs pod/dependent-envars-demo
`
```
```
`
UNCHANGED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
SERVICE\_ADDRESS=https://172.17.0.1:80
ESCAPED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
`
```