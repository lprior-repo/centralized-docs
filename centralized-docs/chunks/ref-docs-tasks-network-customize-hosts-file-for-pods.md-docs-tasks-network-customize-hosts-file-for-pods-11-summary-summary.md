---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#11-summary
chunk_level: summary
chunk_type: prose
heading: Adding additional entries with hostAliases
token_count: 120
summary: You can start a Pod with that configuration by running: ``` `kubectl apply -f https://k8s.io/examples/service/networking/hostaliases-pod.yaml ` ``` ``` `pod/hostaliases-pod created ` ``` Examine a...
---

You can start a Pod with that configuration by running:
```
`kubectl apply -f https://k8s.io/examples/service/networking/hostaliases-pod.yaml
`
```
```
`pod/hostaliases-pod created
`
```
Examine a Pod's details to see its IPv4 address and its status:
```
`kubectl get pod --output=wide
`
```
```
`NAME READY STATUS RESTARTS AGE IP NODE
hostaliases-pod 0/1 Completed 0 6s 10.200.0.5 worker0
`
```