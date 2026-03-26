---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#38-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 59
summary: ``` `kubectl edit clusterrole system:coredns -n kube-system ` ``` Example insertion of EndpointSlices permissions: ``` `... - apiGroups: - discovery.k8s.io resources: - endpointslices verbs: - list -...
---

```
`kubectl edit clusterrole system:coredns -n kube-system
`
```
Example insertion of EndpointSlices permissions:
```
`...
- apiGroups:
- discovery.k8s.io
resources:
- endpointslices
verbs:
- list
- watch
...
`
```