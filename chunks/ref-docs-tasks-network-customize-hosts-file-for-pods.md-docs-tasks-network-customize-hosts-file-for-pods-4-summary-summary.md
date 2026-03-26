---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#4-summary
chunk_level: summary
chunk_type: prose
heading: Default hosts file content
token_count: 124
summary: ## Default hosts file content Start an Nginx Pod which is assigned a Pod IP: ``` `kubectl run nginx --image nginx ` ``` ``` `pod/nginx created ` ``` Examine a Pod IP: ``` `kubectl get pods...
---

## Default hosts file content
Start an Nginx Pod which is assigned a Pod IP:
```
`kubectl run nginx --image nginx
`
```
```
`pod/nginx created
`
```
Examine a Pod IP:
```
`kubectl get pods --output=wide
`
```
```
`NAME READY STATUS RESTARTS AGE IP NODE
nginx 1/1 Running 0 13s 10.200.0.4 worker0
`
```
The hosts file content would look like this:
```
`kubectl exec nginx -- cat /etc/hosts
`
```