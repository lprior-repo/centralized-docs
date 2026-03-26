---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#24-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 96
summary: ### Debugging Services Services provide load balancing across a set of pods. There are several common problems that can make Services not work properly. The following instructions should help debug...
---

### Debugging Services
Services provide load balancing across a set of pods. There are several common problems that can make Services
not work properly. The following instructions should help debug Service problems.
First, verify that there are endpoints for the service. For every Service object,
the apiserver makes one or more `EndpointSlice` resources available.
You can view these resources with:
```
`kubectl get endpointslices -l kubernetes.io/service-name=${SERVICE\_NAME}
`
```