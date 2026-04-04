---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#6-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 98
summary: ``` `kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .status.addresses}}{{printf \"%s: %s\\n\" .type .address}}{{end}}' ` ``` ``` `Hostname: k8s-linuxpool1-34450317-0...
---

```
`kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .status.addresses}}{{printf "%s: %s\\n" .type .address}}{{end}}'
`
```
```
`Hostname: k8s-linuxpool1-34450317-0
InternalIP: 10.0.0.5
InternalIP: 2001:db8:10::5
`
```