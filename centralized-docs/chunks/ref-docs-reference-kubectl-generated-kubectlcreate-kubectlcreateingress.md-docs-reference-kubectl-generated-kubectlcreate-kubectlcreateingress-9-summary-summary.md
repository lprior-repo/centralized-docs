---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#9-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 119
summary: # Create an ingress with two annotations: ingress.annotation1 and ingress.annotations2 kubectl create ingress annotated --class=default --rule=\"foo.com/bar=svc:port\" \\ --annotation...
---

# Create an ingress with two annotations: ingress.annotation1 and ingress.annotations2
kubectl create ingress annotated --class=default --rule="foo.com/bar=svc:port" \\
--annotation ingress.annotation1=foo \\
--annotation ingress.annotation2=bla
# Create an ingress with the same host and multiple paths
kubectl create ingress multipath --class=default \\
--rule="foo.com/=svc:port" \\
--rule="foo.com/admin/=svcadmin:portadmin"
# Create an ingress with multiple hosts and the pathType as Prefix
kubectl create ingress ingress1 --class=default \\