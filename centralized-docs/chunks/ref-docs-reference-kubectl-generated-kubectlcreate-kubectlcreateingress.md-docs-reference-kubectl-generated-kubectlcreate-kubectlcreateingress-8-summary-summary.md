---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#8-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 116
summary: ` # Create a single ingress called 'simple' that directs requests to foo.com/bar to svc # svc1:8080 with a TLS secret \"my-cert\" kubectl create ingress simple...
---

` # Create a single ingress called 'simple' that directs requests to foo.com/bar to svc
# svc1:8080 with a TLS secret "my-cert"
kubectl create ingress simple --rule="foo.com/bar=svc1:8080,tls=my-cert"
# Create a catch all ingress of "/path" pointing to service svc:port and Ingress Class as "otheringress"
kubectl create ingress catch-all --class=otheringress --rule="/path=svc:port"
# Create an ingress with two annotations: ingress.annotation1 and ingress.annotations2