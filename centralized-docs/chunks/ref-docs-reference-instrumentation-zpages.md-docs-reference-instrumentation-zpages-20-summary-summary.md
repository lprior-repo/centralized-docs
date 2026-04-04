---
doc_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages
chunk_id: ref/docs-reference-instrumentation-zpages.md/docs-reference-instrumentation-zpages#20-summary
chunk_level: summary
chunk_type: prose
heading: z-pages
token_count: 113
summary: ``` `{ \"kind\": \"Flagz\", \"apiVersion\": \"config.k8s.io/v1alpha1\", \"metadata\": { \"name\": \"kube-apiserver\" }, \"flags\": { \"advertise-address\": \"192.168.8.4\", \"allow-privileged\": \"true\", \"anonymous-auth\":...
---

```
`{
"kind": "Flagz",
"apiVersion": "config.k8s.io/v1alpha1",
"metadata": {
"name": "kube-apiserver"
},
"flags": {
"advertise-address": "192.168.8.4",
"allow-privileged": "true",
"anonymous-auth": "true",
"authorization-mode": "[Node,RBAC]",
"enable-priority-and-fairness": "true",
"profiling": "true",
"default-watch-cache-size": "100"
}
}
`
```