---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#11-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 87
summary: # Create an ingress with TLS enabled using a specific secret and pathType as Prefix kubectl create ingress ingsecret --class=default \\ --rule=\"foo.com/\*=svc:8080,tls=secret1\" # Create an ingress...
---

# Create an ingress with TLS enabled using a specific secret and pathType as Prefix
kubectl create ingress ingsecret --class=default \\
--rule="foo.com/\*=svc:8080,tls=secret1"
# Create an ingress with a default backend
kubectl create ingress ingdefault --class=default \\
--default-backend=defaultsvc:http \\
--rule="foo.com/\*=svc:8080,tls=secret1"
`
```