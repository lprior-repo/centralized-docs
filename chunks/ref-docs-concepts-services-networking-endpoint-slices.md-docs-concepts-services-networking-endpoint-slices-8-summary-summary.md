---
doc_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices
chunk_id: ref/docs-concepts-services-networking-endpoint-slices.md/docs-concepts-services-networking-endpoint-slices#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 102
summary: ``` `apiVersion: discovery.k8s.io/v1 kind: EndpointSlice metadata: name: example-abc labels: kubernetes.io/service-name: example addressType: IPv4 ports: - name: http protocol: TCP port: 80...
---

```
`apiVersion: discovery.k8s.io/v1
kind: EndpointSlice
metadata:
name: example-abc
labels:
kubernetes.io/service-name: example
addressType: IPv4
ports:
- name: http
protocol: TCP
port: 80
endpoints:
- addresses:
- "10.1.2.3"
conditions:
ready: true
hostname: pod-1
nodeName: node-1
zone: us-west2-a
`
```