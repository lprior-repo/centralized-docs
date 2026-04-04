---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#10-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 120
summary: # Create an ingress with multiple hosts and the pathType as Prefix kubectl create ingress ingress1 --class=default \\ --rule=\"foo.com/path\*=svc:8080\" \\ --rule=\"bar.com/admin\*=svc2:http\" # Create...
---

# Create an ingress with multiple hosts and the pathType as Prefix
kubectl create ingress ingress1 --class=default \\
--rule="foo.com/path\*=svc:8080" \\
--rule="bar.com/admin\*=svc2:http"
# Create an ingress with TLS enabled using the default ingress certificate and different path types
kubectl create ingress ingtls --class=default \\
--rule="foo.com/=svc:https,tls" \\
--rule="foo.com/path/subpath\*=othersvc:8080"
# Create an ingress with TLS enabled using a specific secret and pathType as Prefix