---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#5-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 49
summary: # kubectl create ingress Create an ingress with the specified name ## Synopsis Create an ingress with the specified name. ``` `kubectl create ingress NAME --rule=host/path=service:port[,tls[=secret]]...
---

# kubectl create ingress
Create an ingress with the specified name
## Synopsis
Create an ingress with the specified name.
```
`kubectl create ingress NAME --rule=host/path=service:port[,tls[=secret]]
`
```