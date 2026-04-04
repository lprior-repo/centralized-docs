---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#9-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 118
summary: value: \"$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)\" - name: ESCAPED\_REFERENCE value: \"$$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)\" ` ``` 1. Create a Pod based on that manifest: ``` `kubectl...
---

value: "$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)"
- name: ESCAPED\_REFERENCE
value: "$$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)"
`
```
1. Create a Pod based on that manifest:
```
`kubectl apply -f https://k8s.io/examples/pods/inject/dependent-envars.yaml
`
```
```
`pod/dependent-envars-demo created
`
```
2. List the running Pods:
```
`kubectl get pods dependent-envars-demo
`
```